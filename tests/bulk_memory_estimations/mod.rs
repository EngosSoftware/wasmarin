mod data_drop;
mod elem_drop;
mod memory_copy;
mod memory_fill;
mod memory_grow;
mod memory_init;

struct BenchmarkData {
  lengths: &'static [usize],
  gas: &'static [usize],
  base: usize,
  unit_size: usize,
  unit_cost: usize,
}

fn estimated(length: usize, benchmark_data: &BenchmarkData) -> usize {
  benchmark_data.base + length.div_ceil(benchmark_data.unit_size) * benchmark_data.unit_cost
}

fn values(benchmark_data: &BenchmarkData) {
  for (i, length) in benchmark_data.lengths.iter().enumerate() {
    let gas_measured = benchmark_data.gas[i];
    let gas_estimated = estimated(*length, benchmark_data);
    let gas_estimated_diff = gas_estimated as isize - gas_measured as isize;
    let gas_estimated_diff_percentage = (gas_estimated_diff as f64 / gas_measured as f64) * 100.0;
    println!(
      "{:12} {:12} {:12} {:12} [{:5.1}]",
      length, gas_measured, gas_estimated, gas_estimated_diff, gas_estimated_diff_percentage
    );
  }
}
