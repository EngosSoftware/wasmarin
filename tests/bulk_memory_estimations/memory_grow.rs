use super::*;

const MEMORY_GROW: BenchmarkData = BenchmarkData {
  lengths: &[1, 2, 5, 10, 20, 50, 100, 200, 500, 1000, 2000, 5000, 10000, 20000, 50000, 65535],
  gas: &[
    2307700, 2293200, 2297600, 2288700, 2267300, 2282600, 2281100, 2289900, 2289700, 2277300, 2299200, 2282500, 2289500, 2286400, 2299000, 2321200,
  ],
  base: 2300000,
  unit_size: 8192,
  unit_cost: 32,
};

#[test]
fn estimate() {
  values(&MEMORY_GROW);
}
