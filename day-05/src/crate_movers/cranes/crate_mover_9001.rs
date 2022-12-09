use crate::{
    crate_movers::{
        commands::move_command::MoveCrateCommand,
        strategies::move_crate_strategy::MoveCrateStrategy,
    },
    CrateStack,
};

pub struct CrateMover9001 {
    pub model: String,
}

impl CrateMover9001 {
    pub fn new() -> CrateMover9001 {
        CrateMover9001 {
            model: String::from("Crane Mover 9001"),
        }
    }
}

impl MoveCrateStrategy for CrateMover9001 {
    fn process_move_command(&self, cmd: &MoveCrateCommand, stacks: &mut Vec<CrateStack>) {
        let number_of_items = cmd.crate_quantity;
        let from = cmd.origin_stack_position;
        let to = cmd.target_stack_position;

        let origin = stacks.get_mut(from - 1).unwrap();
        let items_to_move: Vec<String> = origin.pop_range(0..number_of_items);

        println!(
            "Moving {} items ({:?}) from {} to {}",
            number_of_items, items_to_move, from, to
        );

        let target = stacks.get_mut(to - 1).unwrap();
        target.prepend_many(items_to_move);
    }
}
