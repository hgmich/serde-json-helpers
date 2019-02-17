// SPDX-License-Identifier: MIT OR Apache-2.0+

pub(crate) enum StringTransform {
    Lowercase,
    Uppercase,
    PascalCase,
    CamelCase,
    SnakeCase,
    ScreamingSnakeCase,
    KebabCase,
    ScreamingKebabCase,
}

impl StringTransform {
    pub(crate) fn from_str(s: &str) -> Option<StringTransform> {
        use self::StringTransform::*;

        match s {
            "lowercase" => Some(Lowercase),
            "UPPERCASE" => Some(Uppercase),
            "PascalCase" => Some(PascalCase),
            "camelCase" => Some(CamelCase),
            "snake_case" => Some(SnakeCase),
            "SCREAMING_SNAKE_CASE" => Some(ScreamingSnakeCase),
            "kebab-case" => Some(KebabCase),
            "SCREAMING-KEBAB-CASE" => Some(ScreamingKebabCase),
            _ => None,
        }
    }

    pub(crate) fn transform<S: ToString>(&self, input: S) -> String {
        use StringTransform::*;

        let input_string = input.to_string();

        match self {
            Lowercase => Self::transform_lowercase(&*input_string),
            Uppercase => Self::transform_uppercase(&*input_string),
            PascalCase => Self::transform_pascalcase(&*input_string),
            CamelCase => Self::transform_camelcase(&*input_string),
            SnakeCase => Self::transform_snakecase(&*input_string),
            ScreamingSnakeCase => Self::transform_screaming_snakecase(&*input_string),
            KebabCase => Self::transform_kebabcase(&*input_string),
            ScreamingKebabCase => Self::transform_screaming_kebabcase(&*input_string),
        }
    }

    fn split_by_case(input: &str) -> Vec<&str> {
        let split_posns: Vec<usize> = input
            .chars()
            .enumerate()
            .filter_map(|(pos, chr)| {
                if chr.is_ascii_uppercase() {
                    Some(pos)
                } else {
                    None
                }
            })
            .collect();

        let mut out = Vec::new();

        if !(&split_posns).is_empty() {
            let mut last_pos = 0;

            for pos in &split_posns {
                let pos = *pos;
                if pos != last_pos {
                    out.push(&input[last_pos..pos]);
                }

                last_pos = pos;
            }

            out.push(&input[last_pos..]);
        }

        out
    }

    fn transform_lowercase(input: &str) -> String {
        input.to_ascii_lowercase()
    }

    fn transform_uppercase(input: &str) -> String {
        input.to_ascii_uppercase()
    }

    fn transform_pascalcase(input: &str) -> String {
        let parts = Self::split_by_case(input);

        parts
            .into_iter()
            .map(|part| {
                let mut new_part = part.to_ascii_lowercase();

                let s = new_part.get_mut(0..1);
                s.map(|s| {
                    s.to_ascii_uppercase();

                    &*s
                })
                .expect("Failed to get start of string segment");

                new_part
            })
            .collect()
    }

    fn transform_camelcase(input: &str) -> String {
        let mut out = Self::transform_pascalcase(input);

        let s = out.get_mut(0..1);
        s.map(|s| {
            s.to_ascii_lowercase();

            &*s
        })
        .expect("Failed to get start of string");

        out
    }

    fn transform_snakecase(input: &str) -> String {
        let parts = Self::split_by_case(input);

        parts
            .into_iter()
            .map(|s| s.to_ascii_lowercase())
            .collect::<Vec<String>>()
            .join("_")
    }

    fn transform_screaming_snakecase(input: &str) -> String {
        let parts = Self::split_by_case(input);

        parts
            .into_iter()
            .map(|s| s.to_ascii_uppercase())
            .collect::<Vec<String>>()
            .join("_")
    }

    fn transform_kebabcase(input: &str) -> String {
        let parts = Self::split_by_case(input);

        parts
            .into_iter()
            .map(|s| s.to_ascii_lowercase())
            .collect::<Vec<String>>()
            .join("-")
    }

    fn transform_screaming_kebabcase(input: &str) -> String {
        let parts = Self::split_by_case(input);

        parts
            .into_iter()
            .map(|s| s.to_ascii_uppercase())
            .collect::<Vec<String>>()
            .join("-")
    }
}
