pub(crate) struct ConstantId(pub(crate) usize);
pub(crate) struct ExternalSymbolId(pub(crate) usize);
pub(crate) struct CompiledValueShapeId(pub(crate) usize);

pub(crate) struct ParameterSlot(pub(crate) usize);
pub(crate) struct LocalSlot(pub(crate) usize);
pub(crate) struct InstructionIndex(pub(crate) usize);
pub(crate) struct FieldIndex(pub(crate) usize);
pub(crate) struct VariantDiscriminant(pub(crate) usize);

pub(crate) enum NumericKind {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn persistent_physical_ids_construction_and_index() {
        let cid = ConstantId(0);
        let esid = ExternalSymbolId(1);
        let cvsid = CompiledValueShapeId(2);
        let pslot = ParameterSlot(3);
        let lslot = LocalSlot(4);
        let iidx = InstructionIndex(5);
        let fidx = FieldIndex(6);
        let vdisc = VariantDiscriminant(7);

        assert_eq!(cid.0, 0);
        assert_eq!(esid.0, 1);
        assert_eq!(cvsid.0, 2);
        assert_eq!(pslot.0, 3);
        assert_eq!(lslot.0, 4);
        assert_eq!(iidx.0, 5);
        assert_eq!(fidx.0, 6);
        assert_eq!(vdisc.0, 7);
    }

    #[test]
    fn numeric_kind_12_variants() {
        let kinds = [
            NumericKind::Int8,
            NumericKind::Int16,
            NumericKind::Int32,
            NumericKind::Int64,
            NumericKind::Int128,
            NumericKind::Uint8,
            NumericKind::Uint16,
            NumericKind::Uint32,
            NumericKind::Uint64,
            NumericKind::Uint128,
            NumericKind::Float32,
            NumericKind::Float64,
        ];

        assert_eq!(kinds.len(), 12);
        match &kinds[0] {
            NumericKind::Int8 => {}
            _ => panic!("expected Int8"),
        }
        match &kinds[11] {
            NumericKind::Float64 => {}
            _ => panic!("expected Float64"),
        }
    }
}
