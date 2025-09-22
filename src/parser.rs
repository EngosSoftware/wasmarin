use crate::{CodeSectionEntry, Features, Model, WasmarinError, WasmarinResult};
use std::ops::Range;
use std::path::Path;
use wasmparser::Payload;

/// The WebAssembly parser.
pub struct Parser {
  /// The version number found in the WASM file header.
  version: u16,
  /// The WebAssembly encoding format.
  encoding: wasmparser::Encoding,
  /// The WebAssembly header range.
  header_range: Range<usize>,
}

impl Default for Parser {
  /// Creates a new parser with default settings.
  fn default() -> Self {
    Self::new()
  }
}

impl Parser {
  /// Creates a new parser.
  pub fn new() -> Self {
    Self {
      version: 0,
      encoding: wasmparser::Encoding::Module,
      header_range: Range::default(),
    }
  }

  /// Parses WAT file.
  pub fn parse_wat_file(&mut self, file: impl AsRef<Path>) -> WasmarinResult<()> {
    let wasm = wat::parse_file(file).map_err(|e| WasmarinError::new(e.to_string()))?;
    self.parse_wasm_bytes(&wasm)?;
    Ok(())
  }

  /// Parses WAT bytes.
  pub fn parse_wat_bytes(&mut self, data: &[u8]) -> WasmarinResult<()> {
    let wasm = wat::parse_bytes(data).map_err(|e| WasmarinError::new(e.to_string()))?;
    self.parse_wasm_bytes(&wasm)?;
    Ok(())
  }

  /// Parses WAT string.
  pub fn parse_wat_str(&mut self, wat: impl AsRef<str>) -> WasmarinResult<()> {
    let wasm = wat::parse_str(wat).map_err(|e| WasmarinError::new(e.to_string()))?;
    self.parse_wasm_bytes(&wasm)?;
    Ok(())
  }

  /// Parses WASM binary.
  pub fn parse_wasm_bytes<'a>(&mut self, data: &'a [u8]) -> WasmarinResult<Model<'a>> {
    let mut model = Model::default();

    // Validate the input data against requested WebAssembly features.
    let requested_features = Features::new();
    let mut validator = wasmparser::Validator::new_with_features(requested_features.into());
    validator.validate_all(data).map_err(|e| WasmarinError::new(e.to_string()))?;

    let parser = wasmparser::Parser::new(0);
    for payload in parser.parse_all(data) {
      match payload.map_err(|e| WasmarinError::new(e.to_string()))? {
        Payload::Version { num, encoding, range } => {
          self.version = num;
          self.encoding = encoding;
          self.header_range = range;
        }
        Payload::TypeSection(reader) => {
          for item in reader {
            let rec_group = item.map_err(|e| WasmarinError::new(e.to_string()))?;
            model.rec_groups.push(rec_group);
          }
        }
        Payload::ImportSection(reader) => {
          for item in reader {
            let import = item.map_err(|e| WasmarinError::new(e.to_string()))?;
            model.imports.push(import);
          }
        }
        Payload::FunctionSection(reader) => {
          for item in reader {
            let function_index = item.map_err(|e| WasmarinError::new(e.to_string()))?;
            model.function_indexes.push(function_index);
          }
        }
        Payload::TableSection(reader) => {
          for item in reader {
            let table = item.map_err(|e| WasmarinError::new(e.to_string()))?;
            model.tables.push(table);
          }
        }
        Payload::MemorySection(reader) => {
          for item in reader {
            let memory_type = item.map_err(|e| WasmarinError::new(e.to_string()))?;
            model.memory_types.push(memory_type);
          }
        }
        Payload::TagSection(reader) => {
          for item in reader {
            let tag_type = item.map_err(|e| WasmarinError::new(e.to_string()))?;
            model.tag_types.push(tag_type);
          }
        }
        Payload::GlobalSection(reader) => {
          for item in reader {
            let global = item.map_err(|e| WasmarinError::new(e.to_string()))?;
            model.globals.push(global);
          }
        }
        Payload::ExportSection(reader) => {
          for item in reader {
            let export = item.map_err(|e| WasmarinError::new(e.to_string()))?;
            model.exports.push(export);
          }
        }
        Payload::StartSection { func, range: _ } => {
          model.start_function_index = Some(func);
        }
        Payload::ElementSection(reader) => {
          for item in reader {
            let element = item.map_err(|e| WasmarinError::new(e.to_string()))?;
            model.elements.push(element);
          }
        }
        Payload::DataCountSection { count: _, range: _ } => {
          unimplemented!("Payload::DataCountSection");
        }
        Payload::DataSection(reader) => {
          for item in reader {
            let data = item.map_err(|e| WasmarinError::new(e.to_string()))?;
            model.data.push(data);
          }
        }
        Payload::CodeSectionStart { count, range, size } => {
          // Here we know how many functions we'll be receiving as `CodeSectionEntry`,
          // so we can prepare for that, and afterward we can parse and handle each function individually.
          _ = (count, range, size);
        }
        Payload::CodeSectionEntry(body) => {
          let mut code_section_entry = CodeSectionEntry::default();
          let locals_reader = body.get_locals_reader().map_err(|e| WasmarinError::new(e.to_string()))?;
          for item in locals_reader {
            let (local_index, local_val_type) = item.map_err(|e| WasmarinError::new(e.to_string()))?;
            code_section_entry.locals.push((local_index, local_val_type));
          }
          let operators_reader = body.get_operators_reader().map_err(|e| WasmarinError::new(e.to_string()))?;
          for item in operators_reader {
            let operator = item.map_err(|e| WasmarinError::new(e.to_string()))?;
            code_section_entry.operators.push(operator);
          }
          model.code_section_entries.push(code_section_entry);
        }
        Payload::ModuleSection { parser: _, unchecked_range: _ } => {
          // Sections for WebAssembly components.
          unimplemented!("Payload::ModuleSection");
        }
        Payload::InstanceSection(_reader) => {
          unimplemented!("Payload::InstanceSection");
        }
        Payload::CoreTypeSection(_reader) => {
          unimplemented!("Payload::CoreTypeSection");
        }
        Payload::ComponentSection { parser: _, unchecked_range: _ } => {
          unimplemented!("Payload::ComponentSection");
        }
        Payload::ComponentInstanceSection(_reader) => {
          unimplemented!("Payload::ComponentInstanceSection");
        }
        Payload::ComponentAliasSection(_reader) => {
          unimplemented!("Payload::ComponentAliasSection");
        }
        Payload::ComponentTypeSection(_reader) => {
          unimplemented!("Payload::ComponentTypeSection");
        }
        Payload::ComponentCanonicalSection(_reader) => {
          unimplemented!("Payload::ComponentCanonicalSection");
        }
        Payload::ComponentStartSection { start: _, range: _ } => {
          unimplemented!("Payload::ComponentStartSection");
        }
        Payload::ComponentImportSection(_reader) => {
          unimplemented!("Payload::ComponentImportSection");
        }
        Payload::ComponentExportSection(_reader) => {
          unimplemented!("Payload::ComponentExportSection");
        }
        Payload::CustomSection(reader) => {
          model.custom_sections.push((reader.name().to_string(), reader.data().to_vec()));
        }
        Payload::End(length) => {
          // Once we've reached the end of a parser we either resume at the parent parser
          // or the payload iterator is at its end, and we're done.
          _ = length;
        }
        other => {
          // Most likely you'd return an error here, but if you want
          // you can also inspect the raw contents of unknown sections.
          return Err(WasmarinError::new(match other.as_section() {
            Some((id, range)) => format!("Unknown section, id = {}, range = {:?}", id, range),
            None => "Unknown section".to_string(),
          }));
        }
      }
    }
    Ok(model)
  }
}
