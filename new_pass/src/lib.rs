use crate::traversal::{Action, Named, VisResult, Visitor};
use calyx_ir::{self as ir, CellType, Id, NumAttr};
use std::collections::HashSet;
use std::rc::Rc;

#[derive(Default)]
pub struct WhileToRepeat {}

impl Named for WhileToRepeat {
    fn name() -> &'static str {
        "while-to-repeat"
    }

    fn description() -> &'static str {
        "Rewrites a simple while loops with predicatble iterations to repeat loop"
    }
}

impl Visitor for WhileToRepeat {}
