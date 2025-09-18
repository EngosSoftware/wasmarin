use crate::mappings::*;
use crate::metering::*;
use crate::Model;
use std::borrow::Cow;

/// The WebAssembly encoder.
pub struct Encoder {
  /// Metering properties.
  metering: Option<Metering>,
}

impl Default for Encoder {
  fn default() -> Self {
    Self::new(false)
  }
}

impl Encoder {
  /// Creates a new instance of the [Encoder].
  pub fn new(metering: bool) -> Self {
    Self {
      metering: if metering { Some(Metering::default()) } else { None },
    }
  }

  pub fn encode(&mut self, model: Model) -> Vec<u8> {
    // Prepare the WebAssembly module.
    let mut module = wasm_encoder::Module::new();

    // Encode the type section.
    let mut type_section = wasm_encoder::TypeSection::new();
    for rec_group in model.rec_groups {
      let sub_types: Vec<wasm_encoder::SubType> = rec_group.types().cloned().map(map_sub_type).collect();
      if rec_group.is_explicit_rec_group() {
        type_section.ty().rec(sub_types);
      } else {
        for sub_type in &sub_types {
          type_section.ty().subtype(sub_type);
        }
      }
    }

    // Encode the function section.
    let mut function_section = wasm_encoder::FunctionSection::new();
    for function_index in model.function_indexes {
      function_section.function(function_index);
    }

    // Encode the memory section.
    let mut memory_section = wasm_encoder::MemorySection::new();
    for memory_type in model.memory_types {
      memory_section.memory(map_memory_type(memory_type));
    }

    // Encode the global section.
    let mut global_section = wasm_encoder::GlobalSection::new();
    let mut global_index = 0;
    for global in model.globals {
      global_section.global(map_global_type(global.ty), &map_const_expr(global.init_expr));
      global_index += 1;
    }

    // When metering is enabled, create global variables.
    if let Some(metering) = &mut self.metering {
      // Add global variable for remaining points.
      metering.remaining_points = global_index;
      global_section.global(metering.remaining_points_type(), &metering.remaining_points_initial_value());
    }

    // Encode the export section.
    let mut export_section = wasm_encoder::ExportSection::new();
    for export in model.exports {
      export_section.export(export.name, map_export_kind(export.kind), export.index);
    }

    // When metering is enabled, export global variables.
    if let Some(metering) = &mut self.metering {
      export_section.export("wasmarin_metering_remaining_points", wasm_encoder::ExportKind::Global, metering.remaining_points);
    }
    // Encode the code section.
    let mut code_section = wasm_encoder::CodeSection::new();
    for mut code_section_entry in model.code_section_entries {
      let locals: Vec<(u32, wasm_encoder::ValType)> = code_section_entry.locals.drain(..).map(|(index, val_type)| (index, map_val_type(val_type))).collect();
      let mut f = wasm_encoder::Function::new(locals);
      if let Some(metering) = &self.metering {
        // When metering is enabled, extend operators with metering functionality.
        let mut accumulated_cost = 0;
        for operator in code_section_entry.operators {
          accumulated_cost += metering.cost(&operator);
          for op in metering.feed(operator, accumulated_cost) {
            f.instruction(&map_operator(op));
          }
        }
      } else {
        // When metering is disabled, just map operators into instructions.
        for operator in code_section_entry.operators {
          f.instruction(&map_operator(operator));
        }
      }
      code_section.function(&f);
    }

    module
      .section(&type_section)
      .section(&function_section)
      .section(&memory_section)
      .section(&global_section)
      .section(&export_section)
      .section(&code_section);

    // Encode the custom sections.
    for (name, data) in model.custom_sections {
      let custom_section = wasm_encoder::CustomSection {
        name: Cow::Owned(name),
        data: Cow::Owned(data),
      };
      module.section(&custom_section);
    }

    // Extract the encoded Wasm bytes for this module.
    module.finish()
  }
}
