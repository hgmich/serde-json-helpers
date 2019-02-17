// SPDX-License-Identifier: MIT OR Apache-2.0+

mod util;

use serde_json_helpers::serde_enum_string;

test_enum_variant!(TestEnum, ());

#[test]
fn can_serialize_without_transform() {
    assert_eq!(
        &*serde_json::to_string(&TestEnum::Variant).unwrap(),
        serde_json::to_string("Variant").unwrap()
    );

    assert_eq!(
        &*serde_json::to_string(&TestEnum::ExampleVariant).unwrap(),
        serde_json::to_string("ExampleVariant").unwrap()
    );

    assert_eq!(
        &*serde_json::to_string(&TestEnum::Example2).unwrap(),
        serde_json::to_string("Example2").unwrap()
    );
}

test_enum_variant!(TestEnumPrepended, (prepend_enum_name));

#[test]
fn can_serialize_prepended_without_transform() {
    assert_eq!(
        &*serde_json::to_string(&TestEnumPrepended::Variant).unwrap(),
        serde_json::to_string("TestEnumPrependedVariant").unwrap()
    );

    assert_eq!(
        &*serde_json::to_string(&TestEnumPrepended::ExampleVariant).unwrap(),
        serde_json::to_string("TestEnumPrependedExampleVariant").unwrap()
    );

    assert_eq!(
        &*serde_json::to_string(&TestEnumPrepended::Example2).unwrap(),
        serde_json::to_string("TestEnumPrependedExample2").unwrap()
    );
}

test_enum_variant!(TestEnumLowercase, (transform = "lowercase"));

#[test]
fn can_serialize_lowercase() {
    assert_eq!(
        &*serde_json::to_string(&TestEnumLowercase::Variant).unwrap(),
        serde_json::to_string("variant").unwrap()
    );

    assert_eq!(
        &*serde_json::to_string(&TestEnumLowercase::ExampleVariant).unwrap(),
        serde_json::to_string("examplevariant").unwrap()
    );

    assert_eq!(
        &*serde_json::to_string(&TestEnumLowercase::Example2).unwrap(),
        serde_json::to_string("example2").unwrap()
    );
}

test_enum_variant!(TestEnumUppercase, (transform = "UPPERCASE"));

#[test]
fn can_serialize_uppercase() {
    assert_eq!(
        &*serde_json::to_string(&TestEnumUppercase::Variant).unwrap(),
        serde_json::to_string("VARIANT").unwrap()
    );

    assert_eq!(
        &*serde_json::to_string(&TestEnumUppercase::ExampleVariant).unwrap(),
        serde_json::to_string("EXAMPLEVARIANT").unwrap()
    );

    assert_eq!(
        &*serde_json::to_string(&TestEnumUppercase::Example2).unwrap(),
        serde_json::to_string("EXAMPLE2").unwrap()
    );
}

test_enum_variant!(TestEnumSnakeCase, (transform = "snake_case"));

#[test]
fn can_serialize_snake_case() {
    assert_eq!(
        &*serde_json::to_string(&TestEnumSnakeCase::Variant).unwrap(),
        serde_json::to_string("variant").unwrap()
    );

    assert_eq!(
        &*serde_json::to_string(&TestEnumSnakeCase::ExampleVariant).unwrap(),
        serde_json::to_string("example_variant").unwrap()
    );

    assert_eq!(
        &*serde_json::to_string(&TestEnumSnakeCase::Example2).unwrap(),
        serde_json::to_string("example2").unwrap()
    );
}

test_enum_variant!(
    TestEnumScreamingSnakeCase,
    (transform = "SCREAMING_SNAKE_CASE")
);

#[test]
fn can_serialize_screaming_snake_case() {
    assert_eq!(
        &*serde_json::to_string(&TestEnumScreamingSnakeCase::Variant).unwrap(),
        serde_json::to_string("VARIANT").unwrap()
    );

    assert_eq!(
        &*serde_json::to_string(&TestEnumScreamingSnakeCase::ExampleVariant).unwrap(),
        serde_json::to_string("EXAMPLE_VARIANT").unwrap()
    );

    assert_eq!(
        &*serde_json::to_string(&TestEnumScreamingSnakeCase::Example2).unwrap(),
        serde_json::to_string("EXAMPLE2").unwrap()
    );
}

test_enum_variant!(TestEnumKebabCase, (transform = "kebab-case"));

#[test]
fn can_serialize_kebab_case() {
    assert_eq!(
        &*serde_json::to_string(&TestEnumKebabCase::Variant).unwrap(),
        serde_json::to_string("variant").unwrap()
    );

    assert_eq!(
        &*serde_json::to_string(&TestEnumKebabCase::ExampleVariant).unwrap(),
        serde_json::to_string("example-variant").unwrap()
    );

    assert_eq!(
        &*serde_json::to_string(&TestEnumKebabCase::Example2).unwrap(),
        serde_json::to_string("example2").unwrap()
    );
}

test_enum_variant!(
    TestEnumScreamingKebabCase,
    (transform = "SCREAMING-KEBAB-CASE")
);

#[test]
fn can_serialize_screaming_kebab_case() {
    assert_eq!(
        &*serde_json::to_string(&TestEnumScreamingKebabCase::Variant).unwrap(),
        serde_json::to_string("VARIANT").unwrap()
    );

    assert_eq!(
        &*serde_json::to_string(&TestEnumScreamingKebabCase::ExampleVariant).unwrap(),
        serde_json::to_string("EXAMPLE-VARIANT").unwrap()
    );

    assert_eq!(
        &*serde_json::to_string(&TestEnumScreamingKebabCase::Example2).unwrap(),
        serde_json::to_string("EXAMPLE2").unwrap()
    );
}

test_enum_variant!(
    TestEnumPrependedSnakeCase,
    (transform = "snake_case", prepend_enum_name)
);

#[test]
fn can_serialize_prepended_snake_case() {
    assert_eq!(
        &*serde_json::to_string(&TestEnumPrependedSnakeCase::Variant).unwrap(),
        serde_json::to_string("test_enum_prepended_snake_case_variant").unwrap()
    );

    assert_eq!(
        &*serde_json::to_string(&TestEnumPrependedSnakeCase::ExampleVariant).unwrap(),
        serde_json::to_string("test_enum_prepended_snake_case_example_variant").unwrap()
    );

    assert_eq!(
        &*serde_json::to_string(&TestEnumPrependedSnakeCase::Example2).unwrap(),
        serde_json::to_string("test_enum_prepended_snake_case_example2").unwrap()
    );
}
