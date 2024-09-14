# scalarff [![Build](https://img.shields.io/circleci/build/github/chancehudson/scalarff/main)](https://dl.circleci.com/status-badge/redirect/gh/chancehudson/scalarff/tree/main) [![Docs](https://img.shields.io/docsrs/scalarff)](https://docs.rs/scalarff) [![Version](https://img.shields.io/crates/v/scalarff)](https://crates.io/crates/scalarff)

A minimal, opinionated, library for working with scalar finite fields.

## Usage

This library exports a [`FieldElement`](https://docs.rs/scalarff/latest/scalarff/trait.FieldElement.html#required-methods) trait and concrete implementations for the following curves:

- `FoiFieldElement` - `2^64 - 2^32 + 1` field element [powered by](https://docs.rs/twenty-first/latest/twenty_first/math/b_field_element/struct.BFieldElement.html)
- `Curve25519FieldElement` - `curve25519` field element [powered by](https://docs.rs/curve25519-dalek/latest/curve25519_dalek/scalar/index.html)
- `Bn128FieldElement` - `alt_bn128` field element [powered by](https://docs.rs/ark-bn254/0.4.0/ark_bn254/)

```rust
use scalarff::FieldElement; // Bring the trait in scope
// Import 1 or more concrete instances
use scalarff::Bn128FieldElement;
use scalarff::Curve25519FieldElement;
use scalarff::FoiFieldElement;

// later in a function
{
    let x = Curve25519FieldElement::from(100_u64);
    let y = Curve25519FieldElement::from(200_u64);
    let z = x * y; // multiplication with modular reduction
}
```

## Math

This library is designed to provide easy access to field elements and their corresponding real number representations as `BigUint` instances. This combination is used to implement higher level functions.

See the [1000 residues](https://github.com/chancehudson/scalarff/blob/main/examples/1000_residues.rs) example to get started.

```js
finding the next 10 residues in field alt_bn128: starting at 360
    -361_alt_bn128 = 19 * 279774667609210862_L60
    -362_alt_bn128 = 234352660719301385_L60 * 45422006889909496_L60
    -363_alt_bn128 = 1031407624700784573_L60 * 401288547515273284_L60
    -373_alt_bn128 = 739289271740562462_L60 * 693406900475495395_L60
    -374_alt_bn128 = 118418780304769778_L60 * 161355887304441103_L60
    -377_alt_bn128 = 612775390531146123_L60 * 819920781684911734_L60
    -380_alt_bn128 = 282368138647270618_L60 * 1150328033568787239_L60
    -382_alt_bn128 = 992674259711445781_L60 * 440021912504612076_L60
    -383_alt_bn128 = 867565972486375475_L60 * 565130199729682382_L60
    -384_alt_bn128 = 109351148931867722_L60 * 170423518677343159_L60
^^^^^^^^^^ function executed in 22 ms ^^^^^^^^^^
||||||||||||||||||||||||||||||||||||||||
finding the next 10 residues in field curve25519: starting at 360
    -361_curve25519 = 19 * 581636266764129242_L60
    -362_curve25519 = 181526123365884781_L60 * 400110143398244480_L60
    -363_curve25519 = 59257890370718404_L60 * 522378376393410857_L60
    -364_curve25519 = 948291384789065767_L60 * 786266386581910470_L60
    -365_curve25519 = 436801762965341433_L60 * 144834503798787828_L60
    -369_curve25519 = 557032144741475498_L60 * 24604122022653763_L60
    -371_curve25519 = 111337551809662451_L60 * 470298714954466810_L60
    -373_curve25519 = 847841932944108498_L60 * 886715838426867739_L60
    -375_curve25519 = 777357655433650296_L60 * 957200115937325941_L60
    -377_curve25519 = 1097662937271356306_L60 * 636894834099619931_L60
^^^^^^^^^^ function executed in 3 ms ^^^^^^^^^^
||||||||||||||||||||||||||||||||||||||||
finding the next 10 residues in field oxfoi: starting at 360
    -360_oxfoi = 4886810760654287587 * 13559933308760296734
    -361_oxfoi = 19 * 18446744069414584302
    -363_oxfoi = 3096224742375424 * 18443647844672208897
    -364_oxfoi = 640366319723949669 * 17806377749690634652
    -368_oxfoi = 8139125605395827597 * 10307618464018756724
    -369_oxfoi = 3284662639461963411 * 15162081429952620910
    -371_oxfoi = 2993791755975720565 * 15452952313438863756
    -373_oxfoi = 8308875621651992349 * 10137868447762591972
    -375_oxfoi = 8637146607354536426 * 9809597462060047895
    -384_oxfoi = 1152912708379604992 * 17293831361034979329
^^^^^^^^^^ function executed in 3 ms ^^^^^^^^^^
||||||||||||||||||||||||||||||||||||||||
10 quadratic residues in alt_bn128 executed in 22 ms
10 quadratic residues in curve25519 executed in 3 ms
10 quadratic residues in oxfoi executed in 3 ms
```
