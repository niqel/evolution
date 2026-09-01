use alloc::vec::Vec;

use crate::data::lexical::SourceSpan;

pub(crate) struct SourceMap {
    pub(crate) functions: Vec<Vec<SourceSpan>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_map_multi_function_span_indexing() {
        let sm = SourceMap {
            functions: alloc::vec![
                alloc::vec![
                    SourceSpan { start: 0, end: 5 },
                    SourceSpan { start: 5, end: 10 },
                ],
                alloc::vec![
                    SourceSpan { start: 10, end: 20 },
                    SourceSpan { start: 20, end: 35 },
                ],
            ],
        };

        assert_eq!(sm.functions.len(), 2);
        assert_eq!(sm.functions[0].len(), 2);
        assert_eq!(sm.functions[0][0].start, 0);
        assert_eq!(sm.functions[0][0].end, 5);
        assert_eq!(sm.functions[0][1].start, 5);
        assert_eq!(sm.functions[0][1].end, 10);

        assert_eq!(sm.functions[1].len(), 2);
        assert_eq!(sm.functions[1][0].start, 10);
        assert_eq!(sm.functions[1][0].end, 20);
        assert_eq!(sm.functions[1][1].start, 20);
        assert_eq!(sm.functions[1][1].end, 35);
    }
}
