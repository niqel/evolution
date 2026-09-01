pub(crate) struct TypeId(pub(crate) usize);
pub(crate) struct FunctionId(pub(crate) usize);
pub(crate) struct BindingId(pub(crate) usize);
pub(crate) struct FieldId(pub(crate) usize);
pub(crate) struct VariantId(pub(crate) usize);
pub(crate) struct SignatureId(pub(crate) usize);
pub(crate) struct SignatureBindingId(pub(crate) usize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semantic_ids_construction_and_index() {
        let tid = TypeId(0);
        let fid = FunctionId(1);
        let bid = BindingId(2);
        let flid = FieldId(3);
        let vid = VariantId(4);
        let sid = SignatureId(5);
        let sbid = SignatureBindingId(6);

        assert_eq!(tid.0, 0);
        assert_eq!(fid.0, 1);
        assert_eq!(bid.0, 2);
        assert_eq!(flid.0, 3);
        assert_eq!(vid.0, 4);
        assert_eq!(sid.0, 5);
        assert_eq!(sbid.0, 6);
    }
}
