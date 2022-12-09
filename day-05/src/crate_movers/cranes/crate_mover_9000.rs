use crate::{
    crate_movers::{
        commands::move_command::MoveCrateCommand,
        strategies::move_crate_strategy::MoveCrateStrategy,
    },
    CrateStack,
};

pub struct CrateMover9000 {
    pub model: String,
}

impl CrateMover9000 {
    pub fn new() -> CrateMover9000 {
        CrateMover9000 {
            model: String::from("Crane Mover 9000"),
        }
    }
}

impl MoveCrateStrategy for CrateMover9000 {
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
        for item in items_to_move {
            target.push(item);
        }
    }
}
