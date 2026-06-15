use crate::traversal::{Action, Named, VisResult, Visitor};
use calyx_ir::{self as ir, LibrarySignatures};

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

        //   Grab conditional, continue if it does not exist
        let tempConditional = s.cond.as_ref();
        if tempConditional.is_none() {
            return Ok(Action::Continue);
        }
        let combGroup = tempConditional.unwrap();

        //   Grab the port with a read lock
        let cond_port = s.port.borrow();
        // NOTE: Not sure if I need this test to make sure there is an out port
        // if cond_port.name != "out" {
        //     return Ok(Action::Continue);
        // }

        //   The port should be the out of the while loop, this code gets the cell of the parent
        //     EX if lt then the port is lt.out and the cell is lt
        let cond_cell = cond_port.cell_parent();

        //   Release the read lock made earlier
        drop(cond_port);

        // 2. Check if the condition can be simplified to a set number of repeats

        // 3. Replace everything as a static repeat loop if possible

        Ok(Action::Continue)
    }
}

