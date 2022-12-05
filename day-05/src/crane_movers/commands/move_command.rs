use crate::{crane_movers::strategies::move_crane_strategy::MoveCraneStrategy, CrateStack};

pub struct CraneMoverCommand {
    pub crate_quantity: usize,
    pub origin_stack_position: usize,
    pub target_stack_position: usize,
}

impl CraneMoverCommand {
    pub fn new(
        crate_quantity: usize,
        origin_stack_position: usize,
        target_stack_position: usize,
    ) -> Self {
        Self {
            crate_quantity,
            origin_stack_position,
            target_stack_position,
        }
    }

    pub fn apply_using(&self, crane: &impl MoveCraneStrategy, stacks: &mut Vec<CrateStack>) {
        crane.process_move_command(self, stacks);
    }
}
