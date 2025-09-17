#![allow(dead_code)]

use crate::mappings::*;
use crate::Model;

/// The WebAssembly encoder.
pub struct Encoder;

impl Default for Encoder {
  fn default() -> Self {
    Self::new()
  }
}

impl Encoder {
  pub fn new() -> Self {
    Self {}
  }

  pub fn encode(&self, model: Model) -> Vec<u8> {
    // Prepare the WebAssembly module.
    let mut module = wasm_encoder::Module::new();

    // Encode the type section.
    let mut type_section = wasm_encoder::TypeSection::new();
    for rec_group in model.rec_groups {
      let sub_types: Vec<wasm_encoder::SubType> = rec_group.types().map(map_sub_type).collect();
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

    // Encode the export section.
    // let mut exports = ExportSection::new();
    // exports.export("f", ExportKind::Func, 0);
    // module.section(&exports);
    //

    // Encode the code section.
    let mut code_section = wasm_encoder::CodeSection::new();
    for code_section_entry in model.code_section_entries {
      let locals: Vec<(u32, wasm_encoder::ValType)> = code_section_entry.locals.iter().map(|(index, val_type)| (*index, map_val_type(val_type))).collect();
      let mut f = wasm_encoder::Function::new(locals);
      for operator in code_section_entry.operators {
        f.instruction(&map_operator(operator));
      }
      code_section.function(&f);
    }

    module.section(&type_section).section(&function_section).section(&memory_section).section(&code_section);

    // Extract the encoded Wasm bytes for this module.
    module.finish()
  }
}
