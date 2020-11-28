use std::collections::{HashMap, HashSet};

use serde::Serialize;

pub trait Convertible: Serialize {
    type T;
    fn as_seq(&self) -> &[Self::T];
    fn is_vec_of_maps(&self) -> bool {
        false
    }
    fn try_into_records(&self) -> Option<Vec<Vec<String>>> {
        None
    }
}

impl Convertible for serde_json::Value {
    type T = serde_json::Value;

    fn as_seq(&self) -> &[Self::T] {
        self.as_array().unwrap().as_slice()
    }

    fn try_into_records(&self) -> Option<Vec<Vec<String>>> {
        if !self.is_vec_of_maps() {
            return None;
        }

        let header: HashSet<String> = self
            .as_array()?
            .iter()
            .filter_map(|object| object.as_object())
            .flat_map(|object| object.keys())
            .cloned()
            .collect();
        let header: Vec<String> = header.into_iter().collect();

        Some(
            std::iter::once(header)
                .chain(
                    self.as_array()?
                        .iter()
                        .filter_map(|object| object.as_object())
                        .map(|object| {
                            object
                                .values()
                                .filter_map(|v| v.as_str())
                                .map(|v| v.to_string())
                                .collect::<Vec<_>>()
                        }),
                )
                .collect(),
        )
    }

    fn is_vec_of_maps(&self) -> bool {
        match self {
            serde_json::Value::Array(items) => {
                if items.iter().any(|item| match item {
                    serde_json::Value::Object(inner) => {
                        inner.iter().any(|(_, value)| !value.is_string())
                    }
                    _ => true,
                }) {
                    return false;
                }

                let mut header_counts: HashMap<&str, usize> = HashMap::new();
                for item in items.iter().filter_map(|item| item.as_object()) {
                    for k in item.keys() {
                        header_counts
                            .entry(k.as_str())
                            .and_modify(|v| *v = *v + 1)
                            .or_insert(1);
                    }
                }

                if let Some(first_count) =
                    header_counts.values().next().cloned()
                {
                    header_counts.values().all(|v| *v == first_count)
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}

impl Convertible for toml::Value {
    type T = toml::Value;

    fn as_seq(&self) -> &[Self::T] {
        self.as_array().unwrap().as_slice()
    }
}

impl Convertible for ron::Value {
    type T = Self;

    fn as_seq(&self) -> &[Self::T] {
        if let ron::Value::Seq(s) = self {
            s.as_slice()
        } else {
            panic!(":(")
        }
    }
}

impl Convertible for serde_yaml::Value {
    type T = Self;

    fn as_seq(&self) -> &[Self::T] {
        self.as_sequence().unwrap().as_slice()
    }
}

impl Convertible for Vec<Vec<String>> {
    type T = Vec<String>;

    fn as_seq(&self) -> &[Self::T] {
        self.as_slice()
    }
}

impl Convertible for Vec<HashMap<String, String>> {
    type T = Vec<String>;

    fn as_seq(&self) -> &[Self::T] {
        todo!()
    }
}
