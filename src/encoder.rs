use wasm_encoder::{CodeSection, ExportKind, ExportSection, Function, FunctionSection, Module, TypeSection, ValType};

/// The WebAssembly encoder.
pub struct Encoder {
  //
}

#[test]
fn a() {
  let mut module = Module::new();

  // Encode the type section.
  let mut types = TypeSection::new();
  let params = vec![ValType::I32, ValType::I32];
  let results = vec![ValType::I32];
  types.ty().function(params, results);

  module.section(&types);

  // Encode the function section.
  let mut functions = FunctionSection::new();
  let type_index = 0;
  functions.function(type_index);
  module.section(&functions);

  // Encode the export section.
  let mut exports = ExportSection::new();
  exports.export("f", ExportKind::Func, 0);
  module.section(&exports);

  // Encode the code section.
  let mut codes = CodeSection::new();
  let locals = vec![];
  let mut f = Function::new(locals);
  f.instructions().local_get(0).local_get(1).i32_add().end();
  codes.function(&f);
  module.section(&codes);

  // Extract the encoded Wasm bytes for this module.
  let wasm_bytes = module.finish();

  // We generated a valid Wasm module!
  assert!(wasmparser::validate(&wasm_bytes).is_ok());

  let wat = wasmprinter::print_bytes(&wasm_bytes).unwrap();
  println!("{}", wat);
}
