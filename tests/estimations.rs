#[test]
fn estimations() {
  const LOCALS: [usize; 36] = [
    1, 2, 5, 10, 15, 20, 30, 40, 50, 60, 70, 80, 90, 100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000, 10000, 20000, 30000, 40000,
    50000,
  ];
  const TIMES: [f64; 36] = [
    68.445, 68.003, 72.614, 72.058, 73.384, 71.936, 77.208, 78.275, 79.369, 75.077, 76.899, 77.756, 78.504, 78.904, 90.887, 98.764, 109.16, 125.12, 135.00, 153.44, 153.70, 161.81,
    184.73, 288.11, 387.33, 562.77, 776.52, 930.77, 1060.50, 1211.30, 1335.00, 1469.20, 2954.90, 4588.00, 6218.50, 7936.60,
  ];

  for (locals, time) in LOCALS.iter().zip(TIMES) {
    let old_gas_in_second = ((1.0 / (time * 1e-9)) * 1640.0).round();
    let gas_per_locals = gas_for_locals(locals);
    let new_gas_in_second = ((1.0 / (time * 1e-9)) * (1640.0 + gas_per_locals as f64)).round();
    let old_teragas_in_second = old_gas_in_second / 1e12;
    let new_teragas_in_second = new_gas_in_second / 1e12;
    println!(
      "| {:10} | {:10.6} | {:10.6} | {:10} |",
      locals, old_teragas_in_second, new_teragas_in_second, gas_per_locals
    );
  }
}

fn gas_for_locals(locals: &usize) -> usize {
  locals.saturating_sub(29) * 115
}
