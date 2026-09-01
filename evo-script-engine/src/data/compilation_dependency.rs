use alloc::string::String;
use alloc::vec::Vec;
use std::collections::HashMap;

use crate::data::semantic::SignatureSymbol;

#[derive(PartialEq, Eq, Hash)]
pub(crate) struct TypeSymbol {
    pub(crate) module: String,
    pub(crate) name: String,
}

pub(crate) enum CatalogTypeRef {
    Int,
    Float,
    Bool,
    String,
    Dynamic,

    Int8,
    Int16,
    Int32,
    Int64,
    Int128,

    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Uint128,

    Float32,
    Float64,

    Shared(TypeSymbol),
}

pub(crate) enum CatalogType {
    Struct { fields: Vec<CatalogField> },
    Enum { variants: Vec<CatalogVariant> },
}

pub(crate) struct CatalogField {
    pub(crate) name: String,
    pub(crate) type_ref: CatalogTypeRef,
}

pub(crate) enum CatalogVariant {
    Simple {
        name: String,
    },
    Associated {
        name: String,
        type_ref: CatalogTypeRef,
    },
    Structured {
        name: String,
        fields: Vec<CatalogField>,
    },
}

pub(crate) enum CatalogSignatureParameter {
    Value(CatalogTypeRef),
    SignatureDependency(SignatureSymbol),
}

pub(crate) struct CatalogSignature {
    pub(crate) parameters: Vec<CatalogSignatureParameter>,
    pub(crate) result_type: CatalogTypeRef,
}

pub(crate) struct CompilationCatalog {
    pub(crate) types: HashMap<TypeSymbol, CatalogType>,
    pub(crate) signatures: HashMap<SignatureSymbol, CatalogSignature>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;

    #[test]
    fn type_symbol_hash_map_key_lookup() {
        let mut map = HashMap::new();
        let key = TypeSymbol {
            module: "Core".to_string(),
            name: "User".to_string(),
        };
        map.insert(key, 100);

        let lookup_key = TypeSymbol {
            module: "Core".to_string(),
            name: "User".to_string(),
        };
        assert_eq!(map.get(&lookup_key), Some(&100));

        let other_key = TypeSymbol {
            module: "Core".to_string(),
            name: "Account".to_string(),
        };
        assert_eq!(map.get(&other_key), None);
    }

    #[test]
    fn catalog_type_ref_18_variants() {
        let refs = [
            CatalogTypeRef::Int,
            CatalogTypeRef::Float,
            CatalogTypeRef::Bool,
            CatalogTypeRef::String,
            CatalogTypeRef::Dynamic,
            CatalogTypeRef::Int8,
            CatalogTypeRef::Int16,
            CatalogTypeRef::Int32,
            CatalogTypeRef::Int64,
            CatalogTypeRef::Int128,
            CatalogTypeRef::Uint8,
            CatalogTypeRef::Uint16,
            CatalogTypeRef::Uint32,
            CatalogTypeRef::Uint64,
            CatalogTypeRef::Uint128,
            CatalogTypeRef::Float32,
            CatalogTypeRef::Float64,
            CatalogTypeRef::Shared(TypeSymbol {
                module: "Core".to_string(),
                name: "User".to_string(),
            }),
        ];

        assert_eq!(refs.len(), 18);

        match &refs[0] {
            CatalogTypeRef::Int => {}
            _ => panic!("expected Int"),
        }
        match &refs[1] {
            CatalogTypeRef::Float => {}
            _ => panic!("expected Float"),
        }
        match &refs[2] {
            CatalogTypeRef::Bool => {}
            _ => panic!("expected Bool"),
        }
        match &refs[3] {
            CatalogTypeRef::String => {}
            _ => panic!("expected String"),
        }
        match &refs[4] {
            CatalogTypeRef::Dynamic => {}
            _ => panic!("expected Dynamic"),
        }
        match &refs[5] {
            CatalogTypeRef::Int8 => {}
            _ => panic!("expected Int8"),
        }
        match &refs[6] {
            CatalogTypeRef::Int16 => {}
            _ => panic!("expected Int16"),
        }
        match &refs[7] {
            CatalogTypeRef::Int32 => {}
            _ => panic!("expected Int32"),
        }
        match &refs[8] {
            CatalogTypeRef::Int64 => {}
            _ => panic!("expected Int64"),
        }
        match &refs[9] {
            CatalogTypeRef::Int128 => {}
            _ => panic!("expected Int128"),
        }
        match &refs[10] {
            CatalogTypeRef::Uint8 => {}
            _ => panic!("expected Uint8"),
        }
        match &refs[11] {
            CatalogTypeRef::Uint16 => {}
            _ => panic!("expected Uint16"),
        }
        match &refs[12] {
            CatalogTypeRef::Uint32 => {}
            _ => panic!("expected Uint32"),
        }
        match &refs[13] {
            CatalogTypeRef::Uint64 => {}
            _ => panic!("expected Uint64"),
        }
        match &refs[14] {
            CatalogTypeRef::Uint128 => {}
            _ => panic!("expected Uint128"),
        }
        match &refs[15] {
            CatalogTypeRef::Float32 => {}
            _ => panic!("expected Float32"),
        }
        match &refs[16] {
            CatalogTypeRef::Float64 => {}
            _ => panic!("expected Float64"),
        }
        match &refs[17] {
            CatalogTypeRef::Shared(ts) => {
                assert_eq!(ts.module, "Core");
                assert_eq!(ts.name, "User");
            }
            _ => panic!("expected Shared"),
        }
    }

    #[test]
    fn catalog_type_struct_fields_order() {
        let catalog_struct = CatalogType::Struct {
            fields: alloc::vec![
                CatalogField {
                    name: "id".to_string(),
                    type_ref: CatalogTypeRef::Int64,
                },
                CatalogField {
                    name: "name".to_string(),
                    type_ref: CatalogTypeRef::String,
                },
            ],
        };

        match catalog_struct {
            CatalogType::Struct { fields } => {
                assert_eq!(fields.len(), 2);
                assert_eq!(fields[0].name, "id");
                match fields[0].type_ref {
                    CatalogTypeRef::Int64 => {}
                    _ => panic!("expected Int64"),
                }
                assert_eq!(fields[1].name, "name");
                match fields[1].type_ref {
                    CatalogTypeRef::String => {}
                    _ => panic!("expected String"),
                }
            }
            _ => panic!("expected Struct"),
        }
    }

    #[test]
    fn catalog_type_enum_variants_order() {
        let catalog_enum = CatalogType::Enum {
            variants: alloc::vec![
                CatalogVariant::Simple {
                    name: "Pending".to_string(),
                },
                CatalogVariant::Associated {
                    name: "Active".to_string(),
                    type_ref: CatalogTypeRef::Int32,
                },
                CatalogVariant::Structured {
                    name: "Failed".to_string(),
                    fields: alloc::vec![CatalogField {
                        name: "code".to_string(),
                        type_ref: CatalogTypeRef::Int32,
                    }],
                },
            ],
        };

        match catalog_enum {
            CatalogType::Enum { variants } => {
                assert_eq!(variants.len(), 3);
                match &variants[0] {
                    CatalogVariant::Simple { name } => assert_eq!(name, "Pending"),
                    _ => panic!("expected Simple at [0]"),
                }
                match &variants[1] {
                    CatalogVariant::Associated { name, type_ref } => {
                        assert_eq!(name, "Active");
                        match type_ref {
                            CatalogTypeRef::Int32 => {}
                            _ => panic!("expected Int32"),
                        }
                    }
                    _ => panic!("expected Associated at [1]"),
                }
                match &variants[2] {
                    CatalogVariant::Structured { name, fields } => {
                        assert_eq!(name, "Failed");
                        assert_eq!(fields.len(), 1);
                        assert_eq!(fields[0].name, "code");
                    }
                    _ => panic!("expected Structured at [2]"),
                }
            }
            _ => panic!("expected Enum"),
        }
    }

    #[test]
    fn catalog_variant_structured_fields_order() {
        let structured_variant = CatalogVariant::Structured {
            name: "Custom".to_string(),
            fields: alloc::vec![
                CatalogField {
                    name: "tag".to_string(),
                    type_ref: CatalogTypeRef::String,
                },
                CatalogField {
                    name: "priority".to_string(),
                    type_ref: CatalogTypeRef::Int32,
                },
                CatalogField {
                    name: "code".to_string(),
                    type_ref: CatalogTypeRef::Int32,
                },
            ],
        };

        match structured_variant {
            CatalogVariant::Structured { name, fields } => {
                assert_eq!(name, "Custom");
                assert_eq!(fields.len(), 3);
                assert_eq!(fields[0].name, "tag");
                match fields[0].type_ref {
                    CatalogTypeRef::String => {}
                    _ => panic!("expected String"),
                }
                assert_eq!(fields[1].name, "priority");
                match fields[1].type_ref {
                    CatalogTypeRef::Int32 => {}
                    _ => panic!("expected Int32"),
                }
                assert_eq!(fields[2].name, "code");
                match fields[2].type_ref {
                    CatalogTypeRef::Int32 => {}
                    _ => panic!("expected Int32"),
                }
            }
            _ => panic!("expected Structured"),
        }
    }

    #[test]
    fn catalog_signature_parameter_variants_and_mixed_order() {
        let params = alloc::vec![
            CatalogSignatureParameter::Value(CatalogTypeRef::Int32),
            CatalogSignatureParameter::SignatureDependency(SignatureSymbol {
                module: "Math".to_string(),
                name: "Adder".to_string(),
            }),
            CatalogSignatureParameter::Value(CatalogTypeRef::String),
        ];

        assert_eq!(params.len(), 3);
        match &params[0] {
            CatalogSignatureParameter::Value(tr) => match tr {
                CatalogTypeRef::Int32 => {}
                _ => panic!("expected Int32"),
            },
            _ => panic!("expected Value at [0]"),
        }
        match &params[1] {
            CatalogSignatureParameter::SignatureDependency(sym) => {
                assert_eq!(sym.module, "Math");
                assert_eq!(sym.name, "Adder");
            }
            _ => panic!("expected SignatureDependency at [1]"),
        }
        match &params[2] {
            CatalogSignatureParameter::Value(tr) => match tr {
                CatalogTypeRef::String => {}
                _ => panic!("expected String"),
            },
            _ => panic!("expected Value at [2]"),
        }
    }

    #[test]
    fn catalog_signature_parameters_order_and_result_type() {
        let sig = CatalogSignature {
            parameters: alloc::vec![
                CatalogSignatureParameter::Value(CatalogTypeRef::Int32),
                CatalogSignatureParameter::Value(CatalogTypeRef::Int32),
            ],
            result_type: CatalogTypeRef::Int64,
        };

        assert_eq!(sig.parameters.len(), 2);
        match &sig.parameters[0] {
            CatalogSignatureParameter::Value(tr) => match tr {
                CatalogTypeRef::Int32 => {}
                _ => panic!("expected Int32"),
            },
            _ => panic!("expected Value at [0]"),
        }
        match &sig.parameters[1] {
            CatalogSignatureParameter::Value(tr) => match tr {
                CatalogTypeRef::Int32 => {}
                _ => panic!("expected Int32"),
            },
            _ => panic!("expected Value at [1]"),
        }
        match sig.result_type {
            CatalogTypeRef::Int64 => {}
            _ => panic!("expected Int64"),
        }
    }

    #[test]
    fn compilation_catalog_empty_representation() {
        let catalog = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };

        assert_eq!(catalog.types.len(), 0);
        assert_eq!(catalog.signatures.len(), 0);
    }

    #[test]
    fn compilation_catalog_insertion_lookup_and_independent_namespaces() {
        let mut catalog = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };

        let type_sym = TypeSymbol {
            module: "Math".to_string(),
            name: "Compute".to_string(),
        };
        let sig_sym = SignatureSymbol {
            module: "Math".to_string(),
            name: "Compute".to_string(),
        };

        catalog.types.insert(
            type_sym,
            CatalogType::Struct {
                fields: alloc::vec![],
            },
        );

        catalog.signatures.insert(
            sig_sym,
            CatalogSignature {
                parameters: alloc::vec![],
                result_type: CatalogTypeRef::Int32,
            },
        );

        assert_eq!(catalog.types.len(), 1);
        assert_eq!(catalog.signatures.len(), 1);

        let type_lookup = TypeSymbol {
            module: "Math".to_string(),
            name: "Compute".to_string(),
        };
        let sig_lookup = SignatureSymbol {
            module: "Math".to_string(),
            name: "Compute".to_string(),
        };

        let fetched_type = catalog.types.get(&type_lookup);
        assert!(fetched_type.is_some());
        match fetched_type.unwrap() {
            CatalogType::Struct { fields } => assert_eq!(fields.len(), 0),
            _ => panic!("expected Struct"),
        }

        let fetched_sig = catalog.signatures.get(&sig_lookup);
        assert!(fetched_sig.is_some());
        match &fetched_sig.unwrap().result_type {
            CatalogTypeRef::Int32 => {}
            _ => panic!("expected Int32"),
        }
    }
}
