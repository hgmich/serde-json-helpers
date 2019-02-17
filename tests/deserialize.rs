// SPDX-License-Identifier: MIT OR Apache-2.0+

mod util;

use serde_json_helpers::serde_enum_string;

test_enum_variant!(TestEnum, ());

#[test]
fn can_deserialize_without_transform() {
    assert_eq!(
        serde_json::from_str::<TestEnum>("\"Variant\"").unwrap(),
        TestEnum::Variant
    );

    assert_eq!(
        serde_json::from_str::<TestEnum>("\"ExampleVariant\"").unwrap(),
        TestEnum::ExampleVariant
    );

    assert_eq!(
        serde_json::from_str::<TestEnum>("\"Example2\"").unwrap(),
        TestEnum::Example2
    );
}

test_enum_variant!(TestEnumPrepended, (prepend_enum_name));

#[test]
fn can_deserialize_prepended_without_transform() {
    assert_eq!(
        serde_json::from_str::<TestEnumPrepended>("\"TestEnumPrependedVariant\"").unwrap(),
        TestEnumPrepended::Variant
    );

    assert_eq!(
        serde_json::from_str::<TestEnumPrepended>("\"TestEnumPrependedExampleVariant\"").unwrap(),
        TestEnumPrepended::ExampleVariant
    );

    assert_eq!(
        serde_json::from_str::<TestEnumPrepended>("\"TestEnumPrependedExample2\"").unwrap(),
        TestEnumPrepended::Example2
    );
}

test_enum_variant!(TestEnumLowercase, (transform = "lowercase"));

#[test]
fn can_deserialize_lowercase() {
    assert_eq!(
        serde_json::from_str::<TestEnumLowercase>("\"variant\"").unwrap(),
        TestEnumLowercase::Variant
    );

    assert_eq!(
        serde_json::from_str::<TestEnumLowercase>("\"examplevariant\"").unwrap(),
        TestEnumLowercase::ExampleVariant
    );

    assert_eq!(
        serde_json::from_str::<TestEnumLowercase>("\"example2\"").unwrap(),
        TestEnumLowercase::Example2
    );
}

test_enum_variant!(TestEnumUppercase, (transform = "UPPERCASE"));

#[test]
fn can_deserialize_uppercase() {
    assert_eq!(
        serde_json::from_str::<TestEnumUppercase>("\"VARIANT\"").unwrap(),
        TestEnumUppercase::Variant
    );

    assert_eq!(
        serde_json::from_str::<TestEnumUppercase>("\"EXAMPLEVARIANT\"").unwrap(),
        TestEnumUppercase::ExampleVariant
    );

    assert_eq!(
        serde_json::from_str::<TestEnumUppercase>("\"EXAMPLE2\"").unwrap(),
        TestEnumUppercase::Example2
    );
}

test_enum_variant!(TestEnumSnakeCase, (transform = "snake_case"));

#[test]
fn can_serialize_snake_case() {
    assert_eq!(
        serde_json::from_str::<TestEnumSnakeCase>("\"variant\"").unwrap(),
        TestEnumSnakeCase::Variant
    );

    assert_eq!(
        serde_json::from_str::<TestEnumSnakeCase>("\"example_variant\"").unwrap(),
        TestEnumSnakeCase::ExampleVariant
    );

    assert_eq!(
        serde_json::from_str::<TestEnumSnakeCase>("\"example2\"").unwrap(),
        TestEnumSnakeCase::Example2
    );
}

test_enum_variant!(
    TestEnumScreamingSnakeCase,
    (transform = "SCREAMING_SNAKE_CASE")
);

#[test]
fn can_serialize_screaming_snake_case() {
    assert_eq!(
        serde_json::from_str::<TestEnumScreamingSnakeCase>("\"VARIANT\"").unwrap(),
        TestEnumScreamingSnakeCase::Variant
    );

    assert_eq!(
        serde_json::from_str::<TestEnumScreamingSnakeCase>("\"EXAMPLE_VARIANT\"").unwrap(),
        TestEnumScreamingSnakeCase::ExampleVariant
    );

    assert_eq!(
        serde_json::from_str::<TestEnumScreamingSnakeCase>("\"EXAMPLE2\"").unwrap(),
        TestEnumScreamingSnakeCase::Example2
    );
}

test_enum_variant!(TestEnumKebabCase, (transform = "kebab-case"));

#[test]
fn can_serialize_kebab_case() {
    assert_eq!(
        serde_json::from_str::<TestEnumKebabCase>("\"variant\"").unwrap(),
        TestEnumKebabCase::Variant
    );

    assert_eq!(
        serde_json::from_str::<TestEnumKebabCase>("\"example-variant\"").unwrap(),
        TestEnumKebabCase::ExampleVariant
    );

    assert_eq!(
        serde_json::from_str::<TestEnumKebabCase>("\"example2\"").unwrap(),
        TestEnumKebabCase::Example2
    );
}

test_enum_variant!(
    TestEnumScreamingKebabCase,
    (transform = "SCREAMING-KEBAB-CASE")
);

#[test]
fn can_serialize_screaming_kebab_case() {
    assert_eq!(
        serde_json::from_str::<TestEnumScreamingKebabCase>("\"VARIANT\"").unwrap(),
        TestEnumScreamingKebabCase::Variant
    );

    assert_eq!(
        serde_json::from_str::<TestEnumScreamingKebabCase>("\"EXAMPLE-VARIANT\"").unwrap(),
        TestEnumScreamingKebabCase::ExampleVariant
    );

    assert_eq!(
        serde_json::from_str::<TestEnumScreamingKebabCase>("\"EXAMPLE2\"").unwrap(),
        TestEnumScreamingKebabCase::Example2
    );
}

test_enum_variant!(
    TestEnumPrependedSnakeCase,
    (transform = "snake_case", prepend_enum_name)
);

#[test]
fn can_serialize_prepended_snake_case() {
    assert_eq!(
        serde_json::from_str::<TestEnumPrependedSnakeCase>(
            "\"test_enum_prepended_snake_case_variant\""
        )
        .unwrap(),
        TestEnumPrependedSnakeCase::Variant
    );

    assert_eq!(
        serde_json::from_str::<TestEnumPrependedSnakeCase>(
            "\"test_enum_prepended_snake_case_example_variant\""
        )
        .unwrap(),
        TestEnumPrependedSnakeCase::ExampleVariant
    );

    assert_eq!(
        serde_json::from_str::<TestEnumPrependedSnakeCase>(
            "\"test_enum_prepended_snake_case_example2\""
        )
        .unwrap(),
        TestEnumPrependedSnakeCase::Example2
    );
}
