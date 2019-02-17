[![Crates.io](https://img.shields.io/crates/v/serde-json-helpers.svg)](https://crates.io/crates/serde-json-helpers)
[![Build Status](https://circleci.com/gh/holmesmr/serde-json-helpers/tree/master.svg?style=shield)](https://circleci.com/gh/holmesmr/serde-json-helpers/tree/master)

# serde-json-helpers

A collection of procedural macros compatible with stable Rust 2018 that simplify
some common operations not covered by `serde_derive`.

Whilst this crate is named `serde_json_helpers`, `serde_json` is not a requirement
to use it; it's just what the proc macros were intended to be helpful for.

## Examples

### `serde_enum_string`

```rust
use serde_json_helpers::serde_enum_string;

#[serde_enum_string(transform = "snake_case")]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
enum ExampleEnum {
    Option1,
    Option2,
    Option3,
    AnotherOption,
}

fn main() {
    let out1 =
        serde_json::to_string(&ExampleEnum::Option1).expect("Unable to serialize ExampleEnum");
    let out2 =
        serde_json::to_string(&ExampleEnum::Option2).expect("Unable to serialize ExampleEnum");
    let out3 =
        serde_json::to_string(&ExampleEnum::Option3).expect("Unable to serialize ExampleEnum");
    let out4 = serde_json::to_string(&ExampleEnum::AnotherOption)
        .expect("Unable to serialize ExampleEnum");

    println!("Serialized ExampleEnum::Option1: {}", out1);
    println!("Serialized ExampleEnum::Option2: {}", out2);
    println!("Serialized ExampleEnum::Option3: {}", out3);
    println!("Serialized ExampleEnum::AnotherOption: {}", out4);

    let in1 = serde_json::from_str::<ExampleEnum>(&*out1).expect("Unable to deserialize");
    let in2 = serde_json::from_str::<ExampleEnum>(&*out2).expect("Unable to deserialize");
    let in3 = serde_json::from_str::<ExampleEnum>(&*out3).expect("Unable to deserialize");
    let in4 = serde_json::from_str::<ExampleEnum>(&*out4).expect("Unable to deserialize");

    println!("Deserialized {}: {:?}", out1, in1);
    println!("Deserialized {}: {:?}", out2, in2);
    println!("Deserialized {}: {:?}", out3, in3);
    println!("Deserialized {}: {:?}", out4, in4);

    println!(
        "Bad deserialized value {}: {:?}",
        "\"bad_value\"",
        serde_json::from_str::<ExampleEnum>("\"bad_value\"")
    );
}
```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
