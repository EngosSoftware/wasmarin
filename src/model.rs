/// # Intermediate model for parsed WebAssembly

#[derive(Default)]
pub struct CodeSectionEntry<'a> {
  pub locals: Vec<(u32, wasmparser::ValType)>,
  pub operators: Vec<wasmparser::Operator<'a>>,
}

#[derive(Default)]
pub struct Model<'a> {
  pub custom_sections: Vec<(String, Vec<u8>)>,
  pub rec_groups: Vec<wasmparser::RecGroup>,
  pub imports: Vec<wasmparser::Import<'a>>,
  pub function_indexes: Vec<u32>,
  pub tables: Vec<wasmparser::Table<'a>>,
  pub memory_types: Vec<wasmparser::MemoryType>,
  pub globals: Vec<wasmparser::Global<'a>>,
  pub exports: Vec<wasmparser::Export<'a>>,
  pub start_function_index: Option<u32>,
  pub tag_types: Vec<wasmparser::TagType>,
  pub elements: Vec<wasmparser::Element<'a>>,
  pub data: Vec<wasmparser::Data<'a>>,
  pub code_section_entries: Vec<CodeSectionEntry<'a>>,
}
