const LENGTHS: &[usize] = &[
  1, 2, 5, 10, 20, 50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000, 10000, 20000, 30000, 40000, 50000, 60000, 70000, 80000,
  90000, 100000, 200000, 300000, 400000, 500000, 600000, 700000, 800000, 900000, 1000000, 2000000, 3000000, 4000000, 5000000, 6000000, 7000000, 8000000, 9000000, 10000000,
];

const GAS: &[usize] = &[
  392240, 394160, 415000, 396940, 392810, 396080, 418660, 413370, 409130, 408370, 409100, 409660, 409820, 409580, 409120, 409270, 470160, 490180, 486830, 502990, 503400, 509970,
  522660, 529340, 536820, 556270, 561470, 570160, 568800, 572380, 582780, 577060, 577240, 575540, 553270, 521970, 520360, 503140, 529460, 524800, 561770, 547090, 617960, 1020800,
  1169600, 1358000, 841350, 1227600, 1072800, 1551700, 1959300, 2132900,
];

const BASE: usize = 400000;
const UNIT_SIZE: usize = 1024;
const UNIT_COST: usize = 144;

fn estimated(length: usize) -> usize {
  BASE + length.div_ceil(UNIT_SIZE) * UNIT_COST
}

fn regression(length: usize) -> usize {
  ((length as f64 * 1.450275e-01) + 4.749930e+0).round() as usize
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
      "{:12} {:12} {:12} {:12} {:12} [{:5.1}] {:12} [{:5.1}]",
      length, gas_measured, gas_regression, gas_estimated, gas_regression_diff, gas_regression_diff_perc, gas_estimated_diff, gas_estimated_diff_perc
    );
  }
}

#[test]
fn b() {
  println!("{}", estimated(10000000));
}
