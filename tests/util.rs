#[macro_export]
macro_rules! test_enum_variant {
    ($enum_name:ident, $attrs:tt) => {
        #[serde_enum_string$attrs]
        #[derive(Debug, PartialEq, Clone, Copy)]
        enum $enum_name {
            Variant,
            ExampleVariant,
            Example2,
        }
    };
}
