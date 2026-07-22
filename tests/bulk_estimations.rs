const LENGTHS: &[usize] = &[
  1, 2, 5, 10, 20, 50, 100, 200, 500, 1000, 2000, 5000, 10000, 20000, 50000, 100000, 200000, 500000, 1000000, 2000000, 5000000, 10000000,
];

const GAS: &[usize] = &[
  389040, 390110, 391360, 390700, 390100, 394810, 420260, 407550, 407230, 410180, 460790, 517060, 517930, 539100, 555260, 559370, 539500, 462760, 600480, 947310, 559620, 1832900,
];

const BASE: usize = 400000;
const UNIT_COST: usize = 2048;
const UNIT_SIZE: usize = 248;

fn calc(length: usize) -> usize {
  BASE + length.div_ceil(UNIT_SIZE) * UNIT_COST
}

#[test]
fn a() {
  for (i, length) in LENGTHS.iter().enumerate() {
    println!("{:12.1}    {:12.1}", GAS[i], calc(*length))
  }
}
