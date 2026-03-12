// Copyright © Eiger
// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::{
    operator::{MutantInfo, MutationOperator},
    report::{Mutation, Range},
};
use codespan::FileId;
use move_model::model::Loc;
use std::fmt;

pub const OPERATOR_NAME: &str = "resource_operation_replacement";

/// Replaces `exists<T>(addr)` calls with `true` or `false` to test
/// that smart contracts properly verify resource presence/absence.
#[derive(Debug, Clone)]
pub struct Resource {
    loc: Loc,
}

impl Resource {
    pub fn new(loc: Loc) -> Self {
        Self { loc }
    }
}

impl MutationOperator for Resource {
    fn apply(&self, source: &str) -> Vec<MutantInfo> {
        let start = self.loc.span().start().to_usize();
        let end = self.loc.span().end().to_usize();
        let cur_op = &source[start..end];

        vec!["true", "false"]
            .into_iter()
            .map(|op| {
                let mut mutated_source = source.to_string();
                mutated_source.replace_range(start..end, op);
                MutantInfo::new(
                    mutated_source,
                    Mutation::new(
                        Range::new(start, end),
                        OPERATOR_NAME.to_string(),
                        cur_op.to_string(),
                        op.to_string(),
                    ),
                )
            })
            .collect()
    }

    fn get_file_id(&self) -> FileId {
        self.loc.file_id()
    }

    fn name(&self) -> String {
        OPERATOR_NAME.to_string()
    }
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ResourceOperator(location: file id: {:?}, index start: {}, index stop: {})",
            self.loc.file_id(),
            self.loc.span().start(),
            self.loc.span().end()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codespan::Files;

    #[test]
    fn test_apply_resource_operator() {
        let mut files = Files::new();
        let fid = files.add("test", "test");
        let source = "exists<Coin>(addr)";
        let loc = Loc::new(fid, codespan::Span::new(0, source.len() as u32));

        let operator = Resource::new(loc);
        let result = operator.apply(source);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].mutated_source, "true");
        assert_eq!(result[1].mutated_source, "false");
    }

    #[test]
    fn test_apply_resource_operator_in_context() {
        let mut files = Files::new();
        let fid = files.add("test", "test");
        let source = "if (exists<Coin>(addr)) { }";
        // "exists<Coin>(addr)" starts at index 4, ends at 22
        let loc = Loc::new(fid, codespan::Span::new(4, 22));

        let operator = Resource::new(loc);
        let result = operator.apply(source);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].mutated_source, "if (true) { }");
        assert_eq!(result[1].mutated_source, "if (false) { }");
    }

    #[test]
    fn test_get_file_id() {
        let mut files = Files::new();
        let fid = files.add("test", "test");
        let loc = Loc::new(fid, codespan::Span::new(0, 0));
        let operator = Resource::new(loc);
        assert_eq!(operator.get_file_id(), fid);
    }
}
