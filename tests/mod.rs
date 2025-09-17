mod test_features;
#[cfg(feature = "wasmtime")]
mod test_globals;
mod test_parsing_globals;
mod test_round_trip;
mod test_simple_round_trip;
#[cfg(feature = "wasmtime")]
mod test_simple_wasmtime;
