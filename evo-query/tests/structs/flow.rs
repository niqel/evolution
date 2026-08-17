use evo_query::definitions::structs::owned::flow::Flow;

#[test]
fn flow_variants() {
    assert_eq!(Flow::Continue, Flow::Continue);
    assert_eq!(Flow::Stop, Flow::Stop);
    assert_ne!(Flow::Continue, Flow::Stop);
}
