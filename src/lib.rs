mod errors;

pub use crate::errors::{WasmarinError, WasmarinResult};
use std::path::Path;
use wasmparser::Payload;

pub fn parse_wat_file(file: impl AsRef<Path>) -> WasmarinResult<Vec<u8>> {
  wat::parse_file(file).map_err(|e| WasmarinError::new(e.to_string()))
}

pub fn parse_wat_str(wat: impl AsRef<str>) -> WasmarinResult<Vec<u8>> {
  wat::parse_str(wat).map_err(|e| WasmarinError::new(e.to_string()))
}

pub fn parse_wasm(data: &[u8]) -> WasmarinResult<()> {
  let parser = wasmparser::Parser::new(0);
  for payload in parser.parse_all(data) {
    match payload.map_err(|e| WasmarinError::new(e.to_string()))? {
      version @ Payload::Version { .. } => {
        println!("{:?}", version)
      }
      Payload::TypeSection(reader) => {
        println!("TypeSection: {}", reader.count());
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
  Ok(())
}
