use crate::mappings::*;
use crate::metering::*;
use crate::Model;
use std::borrow::Cow;
use wasmparser::{ElementKind, TableInit};

/// The WebAssembly encoder.
pub struct Encoder {
  /// Metering properties.
  metering: Metering,
}

impl Default for Encoder {
  fn default() -> Self {
    Self::new()
  }
}

impl Encoder {
  /// Creates a new [Encoder] instance.
  pub fn new() -> Self {
    Self { metering: Metering::new(false) }
  }

  /// Creates a new [Encoder] instance with metering.
  pub fn new_with_metering() -> Self {
    Self { metering: Metering::new(true) }
  }

  /// Encode the WebAssembly model into WASM binary.
  pub fn encode(&mut self, model: Model) -> Vec<u8> {
    // Prepare the WebAssembly module.
    let mut module = wasm_encoder::Module::new();

    //----------------------------------------------------------------------------------------------
    // TYPE SECTION
    //
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

    //----------------------------------------------------------------------------------------------
    // IMPORT SECTION
    //
    let mut import_section = wasm_encoder::ImportSection::new();
    for import in model.imports {
      import_section.import(import.module, import.name, map_type_ref(import.ty));
    }

    //----------------------------------------------------------------------------------------------
    // FUNCTION SECTION
    //
    let mut function_section = wasm_encoder::FunctionSection::new();
    for function_index in model.function_indexes {
      function_section.function(function_index);
    }

    //----------------------------------------------------------------------------------------------
    // TABLE SECTION
    //
    let mut table_section = wasm_encoder::TableSection::new();
    for table in model.tables {
      match table.init {
        TableInit::RefNull => {
          table_section.table(map_table_type(table.ty));
        }
        TableInit::Expr(const_expr) => {
          table_section.table_with_init(map_table_type(table.ty), &map_const_expr(const_expr));
        }
      }
    }

    //----------------------------------------------------------------------------------------------
    // MEMORY SECTION
    //
    let mut memory_section = wasm_encoder::MemorySection::new();
    for memory_type in model.memory_types {
      memory_section.memory(map_memory_type(memory_type));
    }

    //----------------------------------------------------------------------------------------------
    // TAG SECTION
    //
    let mut tag_section = wasm_encoder::TagSection::new();
    for tag_type in model.tag_types {
      tag_section.tag(map_tag_type(tag_type));
    }

    //----------------------------------------------------------------------------------------------
    // GLOBAL SECTION
    //
    let mut global_section = wasm_encoder::GlobalSection::new();
    for global in model.globals {
      global_section.global(map_global_type(global.ty), &map_const_expr(global.init_expr));
    }
    self.metering.update_global_section(&mut global_section);

    //----------------------------------------------------------------------------------------------
    // EXPORT SECTION
    //
    let mut export_section = wasm_encoder::ExportSection::new();
    for export in model.exports {
      export_section.export(export.name, map_export_kind(export.kind), export.index);
    }
    self.metering.update_export_section(&mut export_section);

    //----------------------------------------------------------------------------------------------
    // START SECTION
    //
    let start_section: Option<wasm_encoder::StartSection> = model.start_function_index.map(|function_index| wasm_encoder::StartSection { function_index });

    //----------------------------------------------------------------------------------------------
    // ELEMENT SECTION
    //
    let mut element_section = wasm_encoder::ElementSection::new();
    for element in model.elements {
      // match element.kind {
      //   ElementKind::Passive => element_section.passive()
      //   ElementKind::Active { .. } => element_section.active()
      //   ElementKind::Declared => element_section.declared()
      // }
    }

    //----------------------------------------------------------------------------------------------
    // CODE SECTION
    //
    let mut code_section = wasm_encoder::CodeSection::new();
    for mut code_section_entry in model.code_section_entries {
      let locals: Vec<(u32, wasm_encoder::ValType)> = code_section_entry.locals.drain(..).map(|(index, val_type)| (index, map_val_type(val_type))).collect();
      let mut function = wasm_encoder::Function::new(locals);
      self.metering.update_function(&mut function, code_section_entry.operators);
      code_section.function(&function);
    }

    module.section(&type_section);
    module.section(&import_section);
    module.section(&function_section);
    module.section(&table_section);
    module.section(&memory_section);
    if !tag_section.is_empty() {
      module.section(&tag_section);
    }
    module.section(&global_section);
    module.section(&export_section);
    if let Some(start_section) = start_section {
      module.section(&start_section);
    }
    module.section(&element_section);
    module.section(&code_section);

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
