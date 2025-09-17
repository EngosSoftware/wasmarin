/// # Intermediate model for parsed WebAssembly

#[derive(Default)]
pub struct Model {
  pub rec_groups: Vec<wasmparser::RecGroup>,
  pub memory_types: Vec<wasmparser::MemoryType>,
  pub function_indexes: Vec<u32>,
}
