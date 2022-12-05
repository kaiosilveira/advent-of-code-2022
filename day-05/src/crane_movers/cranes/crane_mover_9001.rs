use crate::{
    crane_movers::{
        commands::move_command::CraneMoverCommand,
        strategies::move_crane_strategy::MoveCraneStrategy,
    },
    CrateStack,
};

pub struct CraneMover9001 {
    pub model: String,
}

impl CraneMover9001 {
    pub fn new() -> CraneMover9001 {
        CraneMover9001 {
            model: String::from("Crane Mover 9001"),
        }
    }
}

impl MoveCraneStrategy for CraneMover9001 {
    fn process_move_command(&self, cmd: &CraneMoverCommand, stacks: &mut Vec<CrateStack>) {
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
