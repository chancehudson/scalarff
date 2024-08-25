# scalarff

A minimal, opinionated, library for working with scalar finite fields.

## Usage

This library exports a `FieldElement` trait and a set of structs conforming to it.

- `FoiFieldElement` - `2^64 - 2^32 + 1` field element [powered by](https://docs.rs/twenty-first/latest/twenty_first/math/b_field_element/struct.BFieldElement.html)
- `Curve25519FieldElement` - `curve25519` field element [powered by](https://docs.rs/curve25519-dalek/latest/curve25519_dalek/scalar/index.html)
- `Bn128FieldElement` - `alt_bn128` field element [powered by](https://docs.rs/ark-bn254/0.4.0/ark_bn254/)

See the [1000 residues](./examples/1000_residues.rs) example to get started.
