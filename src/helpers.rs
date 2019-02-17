// SPDX-License-Identifier: MIT OR Apache-2.0+

pub(crate) fn is_derive_attr(attr: &syn::Attribute) -> bool {
    !attr.path.segments.is_empty() && &*attr.path.segments[0].ident.to_string() == "derive"
}

pub(crate) fn guard_against_serde_derives(
    macro_name: &str,
    enum_name: &str,
    attrs: &[syn::Attribute],
) {
    for enum_attr in attrs {
        if is_derive_attr(enum_attr) {
            use quote::ToTokens;

            let meta = enum_attr.parse_meta();
            if let Err(e) = meta {
                let attr_name = enum_attr.path.clone().into_token_stream().to_string();
                panic!(
                    "Error parsing #[{}] invocation for {}: {:?}",
                    &*attr_name, &*enum_name, e
                );
            }

            let meta = meta.unwrap();

            let maybe_has_serde_attrs = match meta {
                syn::Meta::List(ref meta_list) => meta_list
                    .nested
                    .iter()
                    .filter(|part| {
                        use syn::Meta::*;
                        use syn::NestedMeta::*;

                        match **part {
                            Meta(Word(ref ident)) => {
                                let ident_str = ident.to_string();

                                &*ident_str == "Serialize" || &*ident_str == "Deserialize"
                            }
                            _ => false,
                        }
                    })
                    .last()
                    .is_some(),
                _ => false,
            };

            if maybe_has_serde_attrs {
                panic!("#[{}] cannot be used in conjunction with #[derive(Serialize)] or #[derive(Deserialize)]. Remove these derive attributes from {}.", macro_name, &*enum_name)
            }
        }
    }
}

pub(crate) fn get_enum_variant_names(syn_enum: &syn::ItemEnum) -> Vec<(syn::Ident, String)> {
    syn_enum
        .variants
        .iter()
        .map(|v| (v.ident.clone(), v.ident.to_string()))
        .collect()
}
