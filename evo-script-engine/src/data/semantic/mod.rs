use alloc::string::String;

pub(crate) mod expressions;
pub(crate) mod ids;
pub(crate) mod structure;

#[derive(PartialEq, Eq, Hash)]
pub(crate) struct SignatureSymbol {
    pub(crate) module: String,
    pub(crate) name: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;
    use std::collections::HashMap;

    #[test]
    fn signature_symbol_hash_map_key_lookup() {
        let mut map = HashMap::new();
        let key = SignatureSymbol {
            module: "Math".to_string(),
            name: "Add".to_string(),
        };
        map.insert(key, 42);

        let lookup_key = SignatureSymbol {
            module: "Math".to_string(),
            name: "Add".to_string(),
        };
        assert_eq!(map.get(&lookup_key), Some(&42));

        let other_key = SignatureSymbol {
            module: "Math".to_string(),
            name: "Sub".to_string(),
        };
        assert_eq!(map.get(&other_key), None);
    }
}
