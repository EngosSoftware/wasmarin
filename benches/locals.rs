#![feature(test)]

extern crate test;
use test::Bencher;

#[bench]
fn _0001(b: &mut Bencher) {
  const WAT: &str = include_str!("../tests/contracts/big_locals.wat");
  let wasm_bytes = wat::parse_str(WAT).unwrap();
  let engine = wasmtime::Engine::default();
  let module = wasmtime::Module::from_binary(&engine, &wasm_bytes).unwrap();
  let mut store = wasmtime::Store::new(&engine, ());
  let instance = wasmtime::Instance::new(&mut store, &module, &[]).unwrap();
  let fun = instance.get_typed_func::<(), i32>(&mut store, "fun").unwrap();
  b.iter(|| _ = fun.call(&mut store, ()).unwrap());
}

/*

RESULTS

Wasmtime Cranelift:

11.780803034855769 ns/iter (+/- 0.446388596754808)
11.813151041666668 ns/iter (+/- 0.4506429036458339)
11.742627749235734 ns/iter (+/- 0.21931724216627124)
11.986234323601973 ns/iter (+/- 0.4662384354440796)
11.816504693800404 ns/iter (+/- 0.2822089607484859)
12.009458188657408 ns/iter (+/- 1.5264351851851838)
11.767931863536006 ns/iter (+/- 0.2758142620584234)
11.906385387073863 ns/iter (+/- 0.5284352805397727)
11.945906929347826 ns/iter (+/- 1.2487094514266293)
11.879755554199217 ns/iter (+/- 0.34536868286132716)

*/
