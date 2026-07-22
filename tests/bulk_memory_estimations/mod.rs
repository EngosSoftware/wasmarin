mod estimate_data_drop;
mod estimate_elem_drop;

struct BenchmarkData {
  lengths: &'static [usize],
  gas: &'static [usize],
  base: usize,
  unit_size: usize,
  unit_cost: usize,
  r_a: f64,
  r_b: f64,
}

fn estimated(length: usize, benchmark_data: &BenchmarkData) -> usize {
  benchmark_data.base + length.div_ceil(benchmark_data.unit_size) * benchmark_data.unit_cost
}

fn regression(length: usize, benchmark_data: &BenchmarkData) -> usize {
  ((length as f64 * benchmark_data.r_a) + benchmark_data.r_b).round() as usize
}

fn values(benchmark_data: &BenchmarkData) {
  for (i, length) in benchmark_data.lengths.iter().enumerate() {
    let gas_measured = benchmark_data.gas[i];
    let gas_regression = regression(*length, benchmark_data);
    let gas_regression_diff = gas_regression as isize - gas_measured as isize;
    let gas_regression_diff_perc = (gas_regression_diff as f64 / gas_measured as f64) * 100.0;
    let gas_estimated = estimated(*length, benchmark_data);
    let gas_estimated_diff = gas_estimated as isize - gas_measured as isize;
    let gas_estimated_diff_perc = (gas_estimated_diff as f64 / gas_measured as f64) * 100.0;
    println!(
      "{:12} {:12} {:12} {:12} {:12} [{:5.1}] {:12} [{:5.1}]",
      length, gas_measured, gas_regression, gas_estimated, gas_regression_diff, gas_regression_diff_perc, gas_estimated_diff, gas_estimated_diff_perc
    );
  }
}
