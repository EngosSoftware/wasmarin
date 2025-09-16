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
    let model = Model::default();

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
          println!("Version: num = {}, encoding = {:?}, range = {:?}", self.version, self.encoding, self.header_range)
        }
        Payload::TypeSection(reader) => {
          for item in reader {
            let rec_group = item.map_err(|e| WasmarinError::new(e.to_string()))?;
            println!("RecGroup: {:?}", rec_group);
            for sub_type in rec_group.types() {
              println!("SubType: {:?}", sub_type);
            }
          }
        }
        Payload::ImportSection(reader) => {
          println!("ImportSection: {}", reader.count());
        }
        Payload::FunctionSection(reader) => {
          println!("FunctionSection: {}", reader.count());
        }
        Payload::TableSection(reader) => {
          println!("TableSection: {}", reader.count());
        }
        Payload::MemorySection(reader) => {
          println!("MemorySection: {}", reader.count());
        }
        Payload::TagSection(reader) => {
          println!("TagSection: {}", reader.count());
        }
        Payload::GlobalSection(reader) => {
          println!("GlobalSection: {}", reader.count());
        }
        Payload::ExportSection(reader) => {
          println!("ExportSection: {}", reader.count());
        }
        Payload::StartSection { func, range } => {
          println!("StartSection: func = {}, range = {:?}", func, range);
        }
        Payload::ElementSection(reader) => {
          println!("ElementSection: {}", reader.count());
        }
        Payload::DataCountSection { count, range } => {
          println!("DataCountSection: count = {}, range = {:?}", count, range);
        }
        Payload::DataSection(reader) => {
          println!("DataSection: {}", reader.count());
        }
        Payload::CodeSectionStart { count, range, size } => {
          // Here we know how many functions we'll be receiving as
          // `CodeSectionEntry`, so we can prepare for that, and afterward
          // we can parse and handle each function individually.
          println!("CodeSectionStart: count = {}, range = {:?}, size = {}", count, range, size);
        }
        Payload::CodeSectionEntry(body) => {
          // Here we can iterate over `body` to parse the function and its locals.
          let locals_reader = body.get_locals_reader().map_err(|e| WasmarinError::new(e.to_string()))?;
          let mut operators_reader = body.get_operators_reader().map_err(|e| WasmarinError::new(e.to_string()))?;
          let mut operator_count: usize = 0;
          while let Ok(_operator) = operators_reader.read() {
            operator_count += 1;
          }
          println!("CodeSectionEntry: locals count = {}, operators count = {}", locals_reader.get_count(), operator_count);
        }
        Payload::ModuleSection { parser: _, unchecked_range } => {
          // Sections for WebAssembly components.
          println!("ModuleSection: unchecked range = {:?}", unchecked_range);
        }
        Payload::InstanceSection(reader) => {
          println!("InstanceSection: {}", reader.count());
        }
        Payload::CoreTypeSection(reader) => {
          println!("CoreTypeSection: {}", reader.count());
        }
        Payload::ComponentSection { parser: _, unchecked_range } => {
          println!("ComponentSection: unchecked range = {:?}", unchecked_range);
        }
        Payload::ComponentInstanceSection(reader) => {
          println!("ComponentInstanceSection: {}", reader.count());
        }
        Payload::ComponentAliasSection(reader) => {
          println!("ComponentAliasSection: {}", reader.count());
        }
        Payload::ComponentTypeSection(reader) => {
          println!("ComponentTypeSection: {}", reader.count());
        }
        Payload::ComponentCanonicalSection(reader) => {
          println!("ComponentCanonicalSection: {}", reader.count());
        }
        Payload::ComponentStartSection { start: _, range } => {
          println!("ComponentStartSection: range = {:?}", range);
        }
        Payload::ComponentImportSection(reader) => {
          println!("ComponentCanonicalSection: {}", reader.count());
        }
        Payload::ComponentExportSection(reader) => {
          println!("CustomSection: {}", reader.count());
        }
        Payload::CustomSection(reader) => {
          println!("CustomSection: {}", reader.name());
        }
        Payload::End(length) => {
          // Once we've reached the end of a parser we either resume at the parent parser
          // or the payload iterator is at its end, and we're done.
          println!("End at {}", length);
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
