use crate::traversal::{Action, Named, VisResult, Visitor};
use calyx_ir::structure;
use calyx_ir::{self as ir, LibrarySignatures};
use calyx_utils::math::bits_needed_for;

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

impl Visitor for WhileToRepeat {
    fn finish_while(
        &mut self,
        s: &mut ir::While,
        comp: &mut ir::Component,
        ctx: &LibrarySignatures,
        _comps: &[ir::Component],
    ) -> VisResult {
        // 1. Extract the condition

        // 2. Check if the condition can be simplified to a set number of repeats

        // 3. Replace everything as a repeat loop if possible

        Ok(Action::Continue)
    }
}
