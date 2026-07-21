fn get_line(x1: usize, y1: usize, x2: usize, y2: usize) -> (f64, f64) {
  let a = (y2 - y1) as f64 / (x2 - x1) as f64;
  let b = y1 as f64 - a * (x1 as f64);
  (a, b)
}

const LENGTHS: &[usize] = &[
  1, 2, 5, 10, 20, 50, 100, 200, 500, 1000, 2000, 5000, 10000, 20000, 50000, 100000, 200000, 500000, 1000000, 2000000, 5000000, 10000000,
];

const GAS: &[usize] = &[
  389040, 390110, 391360, 390700, 390100, 394810, 420260, 407550, 407230, 410180, 460790, 517060, 517930, 539100, 555260, 559370, 539500, 462760, 600480, 947310, 559620, 1832900,
];

#[test]
fn a() {
  let (a, b) = get_line(1, 390000, 10000000, 2000000);
  println!("y = {:.3}x + {:.3}", a, b);

  for (i, length) in LENGTHS.iter().enumerate() {
    let y = a * (*length as f64) + b;
    let diff = GAS[i] as f64 - y;
    println!("{:8}    {:12.1}    {:12.1}", GAS[i], y, diff)
  }
}
