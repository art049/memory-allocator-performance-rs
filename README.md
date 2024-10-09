# The impact of memory allocators on performance in Rust

## Structure

- Allocators implementing the unstable `Allocator` API are under `src/allocators`

- The global allocator implementing `GlobalAlloc` are under `src/global_alloc`

- benchmarks are under `benches`

## Running the benchmarks locally

To use the `allocator_api` feature, you will need the nightly toolchain.

Install cargo-criterion:

```
cargo install cargo-criterion
```

Run the benchmarks:

```
cargo criterion
```
