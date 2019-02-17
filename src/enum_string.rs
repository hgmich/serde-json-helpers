// SPDX-License-Identifier: MIT OR Apache-2.0+

use proc_macro::TokenStream;
use quote::quote;

use crate::string_transform::StringTransform;
use syn::export::TokenStream2;

pub(crate) struct EnumStringOptions {
    transform: Option<StringTransform>,
    prepend_enum_name: bool,
}

impl Default for EnumStringOptions {
    fn default() -> Self {
        Self {
            transform: None,
            prepend_enum_name: false,
        }
    }
}

impl EnumStringOptions {
    pub(crate) fn from_attr_params(params: &[syn::NestedMeta]) -> Self {
        let mut options: EnumStringOptions = Default::default();
        //        let mut
        for param in params {
            use syn::Meta::*;
            use syn::NestedMeta::*;

            match *param {
                Meta(NameValue(ref value)) => {
                    let key_name = value.ident.to_string();
                    match &*key_name {
                        "transform" => {
                            use syn::Lit::*;

                            if let Str(value_lit) = &value.lit {
                                if let Some(transform) =
                                    StringTransform::from_str(&*value_lit.value())
                                {
                                    options.transform = Some(transform);
                                } else {
                                    panic!("'{}' is not a valid string transformation type for 'transform'", &*value_lit.value());
                                }
                            } else {
                                panic!("Invalid paramater passed for 'transform', string expected");
                            }
                        }
                        _ => panic!("Unknown key '{}' in #[serde_enum_string]", &*key_name),
                    }
                }
                Meta(Word(ref ident)) => {
                    let keyword = ident.to_string();
                    match &*keyword {
                        "prepend_enum_name" => {
                            options.prepend_enum_name = true;
                        }
                        _ => panic!("Unknown keyword '{}' in #[serde_enum_string]", &*keyword),
                    }
                }
                ref unknown => panic!(
                    "Unknown syntax element found in #[serde_enum_string]: {:?}",
                    unknown
                ),
            }
        }

        options
    }
}

pub(crate) fn serde_enum_string_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let macro_name = "serde_enum_string";

    let item_input: syn::ItemEnum = syn::parse_macro_input!(item as syn::ItemEnum);
    let attr_input: Vec<syn::NestedMeta> = syn::parse_macro_input!(attr as syn::AttributeArgs);

    // Get type name and identifier
    let enum_ident = &item_input.ident;
    let enum_name = enum_ident.to_string();

    // Deny use of existing serialize/deserialize impls
    crate::helpers::guard_against_serde_derives(macro_name, &*enum_name, &item_input.attrs);

    let options = EnumStringOptions::from_attr_params(&*attr_input);

    let variants = crate::helpers::get_enum_variant_names(&item_input);

    let variants: Vec<(syn::Ident, String)> = variants
        .into_iter()
        .map(|v| {
            if options.prepend_enum_name {
                let mut out = String::new();
                out.push_str(&*enum_name);
                out.push_str(&*v.1);

                (v.0, out)
            } else {
                v
            }
        })
        .map(|v| {
            if let Some(transform) = &options.transform {
                (v.0, transform.transform(v.1))
            } else {
                v
            }
        })
        .collect();

    let mut enum_serialize_mappings = variants
        .clone()
        .into_iter()
        .map(|(variant_ident, s)| {
            quote! {
                 #enum_ident::#variant_ident => #s
            }
        })
        .collect::<Vec<TokenStream2>>();

    enum_serialize_mappings.push(quote! { _ => unreachable!() });

    let serde_ser_impl = quote! {
        impl serde::Serialize for #enum_ident {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer
            {
                let ser_val = match *self {
                    #(#enum_serialize_mappings),*
                };

                serializer.serialize_str(&*ser_val)
            }
        }
    };

    let mut de_visitor_name = enum_name.clone();
    de_visitor_name.push_str("StrVisitor");
    let de_visitor_ident = syn::Ident::new(&*de_visitor_name, enum_ident.span());

    let enum_deserialize_expecting = variants
        .clone()
        .into_iter()
        .map(|(_, s)| format!("\"{}\"", &*s))
        .collect::<Vec<String>>()
        .join(", ");

    let mut enum_deserialize_mappings = variants
        .into_iter()
        .map(|(variant_ident, s)| {
            quote! {
                 #s => Ok(#enum_ident::#variant_ident)
            }
        })
        .collect::<Vec<TokenStream2>>();

    enum_deserialize_mappings.push(quote! {
        _ => Err(serde::de::Error::invalid_value(
                        serde::de::Unexpected::Str(s),
                        &self,
             ))
    });

    let serde_de_impl = quote! {
        struct #de_visitor_ident;

        impl<'de> serde::Deserialize<'de> for #enum_ident {
            fn deserialize<D>(deserializer: D) -> Result<#enum_ident, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                deserializer.deserialize_str(#de_visitor_ident)
            }
        }

        impl<'de> serde::de::Visitor<'de> for #de_visitor_ident {
            type Value = #enum_ident;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("one of: ")?;
                formatter.write_str(#enum_deserialize_expecting)
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match s {
                    #(#enum_deserialize_mappings),*
                }
            }
        }
    };

    let output = quote! {
        #item_input
        #serde_ser_impl
        #serde_de_impl
    };

    output.into()
}
