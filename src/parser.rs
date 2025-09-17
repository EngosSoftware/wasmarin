use crate::{Features, Model, WasmarinError, WasmarinResult};
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
    self.parse_wasm(&wasm)?;
    Ok(())
  }

  /// Parses WAT bytes.
  pub fn parse_wat_bytes(&mut self, data: &[u8]) -> WasmarinResult<()> {
    let wasm = wat::parse_bytes(data).map_err(|e| WasmarinError::new(e.to_string()))?;
    self.parse_wasm(&wasm)?;
    Ok(())
  }

  /// Parses WAT string.
  pub fn parse_wat_str(&mut self, wat: impl AsRef<str>) -> WasmarinResult<()> {
    let wasm = wat::parse_str(wat).map_err(|e| WasmarinError::new(e.to_string()))?;
    self.parse_wasm(&wasm)?;
    Ok(())
  }

  /// Parses WASM binary.
  pub fn parse_wasm(&mut self, data: &[u8]) -> WasmarinResult<Model> {
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
          // println!("Version: num = {}, encoding = {:?}, range = {:?}", self.version, self.encoding, self.header_range)
        }
        Payload::TypeSection(reader) => {
          for item in reader {
            let rec_group = item.map_err(|e| WasmarinError::new(e.to_string()))?;
            model.rec_groups.push(rec_group);
          }
        }
        Payload::ImportSection(_reader) => {
          // println!("ImportSection: {}", reader.count());
        }
        Payload::FunctionSection(reader) => {
          for item in reader {
            let function_index = item.map_err(|e| WasmarinError::new(e.to_string()))?;
            model.function_indexes.push(function_index);
          }
        }
        Payload::TableSection(_reader) => {
          // println!("TableSection: {}", reader.count());
        }
        Payload::MemorySection(reader) => {
          for item in reader {
            let memory_type = item.map_err(|e| WasmarinError::new(e.to_string()))?;
            model.memory_types.push(memory_type);
          }
        }
        Payload::TagSection(_reader) => {
          // println!("TagSection: {}", reader.count());
        }
        Payload::GlobalSection(_reader) => {
          // println!("GlobalSection: {}", reader.count());
        }
        Payload::ExportSection(_reader) => {
          // println!("ExportSection: {}", reader.count());
        }
        Payload::StartSection { func: _, range: _ } => {
          // println!("StartSection: func = {}, range = {:?}", func, range);
        }
        Payload::ElementSection(_reader) => {
          // println!("ElementSection: {}", reader.count());
        }
        Payload::DataCountSection { count: _, range: _ } => {
          // println!("DataCountSection: count = {}, range = {:?}", count, range);
        }
        Payload::DataSection(_reader) => {
          // println!("DataSection: {}", reader.count());
        }
        Payload::CodeSectionStart { count: _, range: _, size: _ } => {
          // Here we know how many functions we'll be receiving as
          // `CodeSectionEntry`, so we can prepare for that, and afterward
          // we can parse and handle each function individually.
          // println!("CodeSectionStart: count = {}, range = {:?}, size = {}", count, range, size);
        }
        Payload::CodeSectionEntry(body) => {
          // Here we can iterate over `body` to parse the function and its locals.
          let _locals_reader = body.get_locals_reader().map_err(|e| WasmarinError::new(e.to_string()))?;
          let _operators_reader = body.get_operators_reader().map_err(|e| WasmarinError::new(e.to_string()))?;
          // println!("CodeSectionEntry: locals count = {}, operators count = {}", locals_reader.get_count(), operator_count);
        }
        Payload::ModuleSection { parser: _, unchecked_range: _ } => {
          // Sections for WebAssembly components.
          // println!("ModuleSection: unchecked range = {:?}", unchecked_range);
        }
        Payload::InstanceSection(_reader) => {
          // println!("InstanceSection: {}", reader.count());
        }
        Payload::CoreTypeSection(_reader) => {
          // println!("CoreTypeSection: {}", reader.count());
        }
        Payload::ComponentSection { parser: _, unchecked_range: _ } => {
          // println!("ComponentSection: unchecked range = {:?}", unchecked_range);
        }
        Payload::ComponentInstanceSection(_reader) => {
          // println!("ComponentInstanceSection: {}", reader.count());
        }
        Payload::ComponentAliasSection(_reader) => {
          // println!("ComponentAliasSection: {}", reader.count());
        }
        Payload::ComponentTypeSection(_reader) => {
          // println!("ComponentTypeSection: {}", reader.count());
        }
        Payload::ComponentCanonicalSection(_reader) => {
          // println!("ComponentCanonicalSection: {}", reader.count());
        }
        Payload::ComponentStartSection { start: _, range: _ } => {
          // println!("ComponentStartSection: range = {:?}", range);
        }
        Payload::ComponentImportSection(_reader) => {
          // println!("ComponentCanonicalSection: {}", reader.count());
        }
        Payload::ComponentExportSection(_reader) => {
          // println!("CustomSection: {}", reader.count());
        }
        Payload::CustomSection(_reader) => {
          // println!("CustomSection: {}", reader.name());
        }
        Payload::End(_length) => {
          // Once we've reached the end of a parser we either resume at the parent parser
          // or the payload iterator is at its end, and we're done.
          // println!("End at {}", length);
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
