use std::io::Write;
use std::path::Path;
use wasmarin::WasmarinResult;

fn main() -> WasmarinResult<()> {
  let args = std::env::args().skip(1).collect::<Vec<String>>();
  if args.len() == 1 {
    let (sum, max) = count_totals(&args[0]);
    println!("    max locals = {}", max);
    println!("  total locals = {}", sum);
  }
  Ok(())
}

/// Returns (total max, total sum) of locals in multiple contracts.
fn count_totals(dir: impl AsRef<Path>) -> (usize, usize) {
  let mut total_max = 0;
  let mut total_sum = 0;
  let mut dots = 1_usize;
  let mut line = 1_usize;
  print!("{:4} ", line);
  std::io::stdout().flush().unwrap();
  for entry in std::fs::read_dir(dir).unwrap() {
    let entry = entry.unwrap();
    let path = entry.path();
    if path.is_file() {
      let (max, sum) = count_locals(path);
      total_max = total_max.max(max);
      total_sum = total_sum.max(sum);
      if dots.is_multiple_of(100) {
        println!(" max = {}, sum = {}", total_max, total_sum);
        line += 1;
        print!("{:4} ", line);
        std::io::stdout().flush().unwrap();
        dots = 1;
      } else {
        print!(".");
        std::io::stdout().flush().unwrap();
        dots += 1;
      }
    }
  }
  (total_max, total_sum)
}

/// Returns (max, sum) of locals in a contract.
fn count_locals(wasm_file: impl AsRef<Path>) -> (usize, usize) {
  let data = std::fs::read(wasm_file).unwrap();
  let mut parser = wasmarin::Parser::new();
  let model = parser.parse_wasm_bytes(&data).unwrap();
  let mut max = 0;
  let mut sum = 0;
  for code_section_entry in &model.code_section_entries {
    let count = code_section_entry.locals.len();
    sum += count;
    max = count.max(max);
  }
  (max, sum)
}
