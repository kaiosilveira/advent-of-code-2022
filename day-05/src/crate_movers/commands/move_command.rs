use crate::{crane_movers::strategies::move_crate_strategy::MoveCrateStrategy, CrateStack};

pub struct MoveCrateCommand {
    pub crate_quantity: usize,
    pub origin_stack_position: usize,
    pub target_stack_position: usize,
}

impl MoveCrateCommand {
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

    pub fn apply_using(&self, crane: &impl MoveCrateStrategy, stacks: &mut Vec<CrateStack>) {
        crane.process_move_command(self, stacks);
    }
}
