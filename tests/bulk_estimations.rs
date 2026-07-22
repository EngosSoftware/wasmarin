const LENGTHS: &[usize] = &[
  1, 2, 5, 10, 20, 50, 100, 200, 500, 1000, 2000, 5000, 10000, 20000, 50000, 100000, 200000, 500000, 1000000, 2000000, 5000000, 10000000,
];

const GAS: &[usize] = &[
  389040, 390110, 391360, 390700, 390100, 394810, 420260, 407550, 407230, 410180, 460790, 517060, 517930, 539100, 555260, 559370, 539500, 462760, 600480, 947310, 559620, 1832900,
];

const BASE: usize = 400000;
const UNIT_COST: usize = 2048;
const UNIT_SIZE: usize = 248;

fn estimated(length: usize) -> usize {
  BASE + length.div_ceil(UNIT_SIZE) * UNIT_COST
}

fn regression(length: usize) -> usize {
  ((length as f64 * 1.200047e-01) + 4.462121e+05).round() as usize
}

#[test]
fn a() {
  for (i, length) in LENGTHS.iter().enumerate() {
    let gas_measured = GAS[i];
    let gas_regression = regression(*length);
    let gas_regression_diff = gas_regression as isize - gas_measured as isize;
    let gas_regression_diff_perc = (gas_regression_diff as f64 / gas_measured as f64) * 100.0;
    let gas_estimated = estimated(*length);
    let gas_estimated_diff = gas_estimated as isize - gas_measured as isize;
    let gas_estimated_diff_perc = (gas_estimated_diff as f64 / gas_measured as f64) * 100.0;
    println!(
      "{:12} {:12} {:12} {:12} [{:5.1}] {:12} [{:5.1}]",
      gas_measured, gas_regression, gas_estimated, gas_regression_diff, gas_regression_diff_perc, gas_estimated_diff, gas_estimated_diff_perc
    );
  }
}
