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

  pub fn encode(&self, model: &Model) -> Vec<u8> {
    let mut module = wasm_encoder::Module::new();

    //----------------------------------------------------------------------------------------------
    // Encode the type section.
    //----------------------------------------------------------------------------------------------
    let mut type_section = wasm_encoder::TypeSection::new();
    for rec_group in &model.rec_groups {
      let sub_types: Vec<wasm_encoder::SubType> = rec_group.types().map(map_sub_type).collect();
      if rec_group.is_explicit_rec_group() {
        type_section.ty().rec(sub_types);
      } else {
        for sub_type in &sub_types {
          type_section.ty().subtype(sub_type);
        }
      }
    }
    module.section(&type_section);

    //----------------------------------------------------------------------------------------------
    // Encode the memory section.
    //----------------------------------------------------------------------------------------------
    let mut memory_section = wasm_encoder::MemorySection::new();
    for memory_type in &model.memory_types {
      memory_section.memory(map_memory_type(memory_type));
    }
    module.section(&memory_section);

    //----------------------------------------------------------------------------------------------
    // Encode the function section.
    //----------------------------------------------------------------------------------------------
    let mut function_section = wasm_encoder::FunctionSection::new();
    for function_index in &model.function_indexes {
      function_section.function(*function_index);
    }
    // module.section(&function_section);

    // // Encode the function section.
    // let mut functions = FunctionSection::new();
    // let type_index = 0;
    // functions.function(type_index);
    // module.section(&functions);
    //
    // // Encode the export section.
    // let mut exports = ExportSection::new();
    // exports.export("f", ExportKind::Func, 0);
    // module.section(&exports);
    //
    // // Encode the code section.
    // let mut codes = CodeSection::new();
    // let locals = vec![];
    // let mut f = Function::new(locals);
    // f.instructions().local_get(0).local_get(1).i32_add().end();
    // codes.function(&f);
    // module.section(&codes);

    // Extract the encoded Wasm bytes for this module.
    module.finish()
  }
}
