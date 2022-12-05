pub struct CraneMoverCommand {
  crate_quantity: usize,
  origin_stack_position: usize,
  target_stack_position: usize,
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
}

pub trait MoveCraneStrategy {
  fn process_move_command(&self, cmd: &CraneMoverCommand, stacks: &mut Vec<Vec<String>>);
}

pub struct CraneMover9000 {
  pub model: String,
}

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

impl CraneMover9000 {
  pub fn new() -> CraneMover9000 {
      CraneMover9000 {
          model: String::from("Crane Mover 9000"),
      }
  }
}

impl MoveCraneStrategy for CraneMover9000 {
  fn process_move_command(&self, cmd: &CraneMoverCommand, stacks: &mut Vec<Vec<String>>) {
      let mv = cmd.crate_quantity;
      let from = cmd.origin_stack_position;
      let to = cmd.target_stack_position;

      let origin = stacks.get_mut(from - 1).unwrap();
      let items_to_move: Vec<String> = origin.drain(0..mv.clone()).collect();

      println!(
          "Moving {} items ({:?}) from {} to {}",
          mv, items_to_move, from, to
      );

      let target = stacks.get_mut(to - 1).unwrap();
      for item in items_to_move {
          target.insert(0, item);
      }
  }
}

impl MoveCraneStrategy for CraneMover9001 {
  fn process_move_command(&self, cmd: &CraneMoverCommand, stacks: &mut Vec<Vec<String>>) {
      let mv = cmd.crate_quantity;
      let from = cmd.origin_stack_position;
      let to = cmd.target_stack_position;

      let origin = stacks.get_mut(from - 1).unwrap();
      let items_to_move: Vec<String> = origin.drain(0..mv.clone()).collect();

      println!(
          "Moving {} items ({:?}) from {} to {}",
          mv, items_to_move, from, to
      );

      let target = stacks.get_mut(to - 1).unwrap();
      target.splice(0..0, items_to_move);
  }
}
