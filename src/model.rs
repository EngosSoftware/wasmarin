/// # Intermediate model for parsed WebAssembly

#[derive(Default)]
pub struct CodeSectionEntry<'a> {
  pub locals: Vec<(u32, wasmparser::ValType)>,
  pub operators: Vec<wasmparser::Operator<'a>>,
}

#[derive(Default)]
pub struct Model<'a> {
  pub rec_groups: Vec<wasmparser::RecGroup>,
  pub function_indexes: Vec<u32>,
  pub memory_types: Vec<wasmparser::MemoryType>,
  pub globals: Vec<wasmparser::Global<'a>>,
  pub exports: Vec<wasmparser::Export<'a>>,
  pub code_section_entries: Vec<CodeSectionEntry<'a>>,
}
