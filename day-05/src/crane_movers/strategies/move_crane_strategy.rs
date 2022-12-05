use crate::crane_movers::commands::move_command::CraneMoverCommand;

pub trait MoveCraneStrategy {
    fn process_move_command(&self, cmd: &CraneMoverCommand, stacks: &mut Vec<Vec<String>>);
}
