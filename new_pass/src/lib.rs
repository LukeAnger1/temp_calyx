use crate::traversal::{Action, Named, VisResult, Visitor};
use calyx_ir::{self as ir, CellType, Id, LibrarySignatures};
use std::collections::HashSet;

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

        // TODO: Figure add in check that it is lt, short circuit with continue otherwise
        //   I need to seperate into lt.left, lt.right, and lt.out and make sure the conditions match perfectly with what I am replacing

        //    Scan the comb group's assignments for the condition that the right side is a constant
        let mut bound = None;
        for assign in combGroup.borrow().assignments.iter() {
            let src = assign.src.borrow();
            let prototype = src.cell_parent().borrow().prototype.clone();

            // Check to make sure it is a constant on the other port
            if let CellType::Constant { val, .. } = prototype {
                bound = Some(val);
                break;
            }
        }

        // Make sure we found a constant bound while extracting the number of repeats
        let Some(num_repeats) = bound else {
            return Ok(Action::Continue);
        };

        // 3. Replace the while loop with static repeat
        let comb_group_name = combGroup.borrow().name();
        let body = s.body.take_control();

        // Only keep the comb group that is needed so other passes dont fail
        comp.comb_groups
            .retain(|cg| cg.borrow().name() != comb_group_name);
        Ok(Action::change(ir::Control::repeat(
            num_repeats,
            Box::new(body),
        )))
    }

    fn finish(
        &mut self,
        comp: &mut ir::Component,
        _sigs: &LibrarySignatures,
        _comps: &[ir::Component],
    ) -> VisResult {
        // Dont need to do the cleaning here because it is handle earlier
        Ok(Action::Continue)
    }
}
