use alloc::vec::Vec;

use crate::data::compiled::identities::NumericKind;

pub(crate) enum EqualityRule {
    Numeric(NumericKind),
    Boolean,
    String,
    Composite(CompositeEqualityPlan),
}

pub(crate) enum CompositeEqualityPlan {
    Struct {
        fields: Vec<EqualityRule>,
    },
    Enum {
        variants: Vec<EnumEqualityPayloadPlan>,
    },
}

pub(crate) enum EnumEqualityPayloadPlan {
    Simple,
    Associated(EqualityRule),
    Structured { fields: Vec<EqualityRule> },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equality_plans_variants_and_nested_representation() {
        let nested_plan = CompositeEqualityPlan::Struct {
            fields: alloc::vec![
                EqualityRule::Numeric(NumericKind::Int32),
                EqualityRule::String,
                EqualityRule::Composite(CompositeEqualityPlan::Enum {
                    variants: alloc::vec![
                        EnumEqualityPayloadPlan::Simple,
                        EnumEqualityPayloadPlan::Associated(EqualityRule::Boolean),
                        EnumEqualityPayloadPlan::Structured {
                            fields: alloc::vec![EqualityRule::Numeric(NumericKind::Float64),],
                        },
                    ],
                }),
            ],
        };

        match nested_plan {
            CompositeEqualityPlan::Struct { fields } => {
                assert_eq!(fields.len(), 3);
                match &fields[0] {
                    EqualityRule::Numeric(NumericKind::Int32) => {}
                    _ => panic!("expected Numeric(Int32)"),
                }
                match &fields[1] {
                    EqualityRule::String => {}
                    _ => panic!("expected String"),
                }
                match &fields[2] {
                    EqualityRule::Composite(CompositeEqualityPlan::Enum { variants }) => {
                        assert_eq!(variants.len(), 3);
                        match &variants[0] {
                            EnumEqualityPayloadPlan::Simple => {}
                            _ => panic!("expected Simple"),
                        }
                        match &variants[1] {
                            EnumEqualityPayloadPlan::Associated(rule) => match rule {
                                EqualityRule::Boolean => {}
                                _ => panic!("expected Boolean"),
                            },
                            _ => panic!("expected Associated"),
                        }
                        match &variants[2] {
                            EnumEqualityPayloadPlan::Structured { fields } => {
                                assert_eq!(fields.len(), 1);
                                match &fields[0] {
                                    EqualityRule::Numeric(NumericKind::Float64) => {}
                                    _ => panic!("expected Numeric(Float64)"),
                                }
                            }
                            _ => panic!("expected Structured"),
                        }
                    }
                    _ => panic!("expected Composite(Enum)"),
                }
            }
            _ => panic!("expected Struct"),
        }
    }
}
