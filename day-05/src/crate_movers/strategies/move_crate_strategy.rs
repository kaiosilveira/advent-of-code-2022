use crate::{crate_movers::commands::move_command::MoveCrateCommand, CrateStack};

pub trait MoveCrateStrategy {
    fn process_move_command(&self, cmd: &MoveCrateCommand, stacks: &mut Vec<CrateStack>);
}
