# Supply Stacks

Challenge URL: https://adventofcode.com/2022/day/5

The expedition can depart as soon as the final supplies have been unloaded from the ships. Supplies are stored in stacks of marked crates, but because the needed supplies are buried under many other crates, the crates need to be rearranged.

The ship has a giant cargo crane capable of moving crates between stacks. To ensure none of the crates get crushed or fall over, the crane operator will rearrange them in a series of carefully-planned steps. After the crates are rearranged, the desired crates will be at the top of each stack.

## Part one: moving crates

The Elves don't want to interrupt the crane operator during this delicate procedure, but they forgot to ask her which crate will end up where, and they want to be ready to unload them as soon as possible so they can embark.
They do, however, have a drawing of the starting stacks of crates and the rearrangement procedure (your puzzle input). For example:

```
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
```

In this example, there are three stacks of crates. Stack 1 contains two crates: crate `Z` is on the bottom, and crate `N` is on top. Stack 2 contains three crates; from bottom to top, they are crates `M`, `C`, and `D`. Finally, stack 3 contains a single crate, `P`.

Then, the rearrangement procedure is given. In each step of the procedure, a quantity of crates is moved from one stack to a different stack. In the first step of the above rearrangement procedure, one crate is moved from stack 2 to stack 1, resulting in this configuration:

```
[D]
[N] [C]
[Z] [M] [P]
 1   2   3
```

In the second step, three crates are moved from stack 1 to stack 3. Crates are moved one at a time, so the first crate to be moved (`D`) ends up below the second and third crates:

```
        [Z]
        [N]
    [C] [D]
    [M] [P]
 1   2   3
```

Then, both crates are moved from stack 2 to stack 1. Again, because crates are moved one at a time, crate `C` ends up below crate `M`:

```
        [Z]
        [N]
[M]     [D]
[C]     [P]
 1   2   3
```

Finally, one crate is moved from stack 1 to stack 2:

```
        [Z]
        [N]
        [D]
[C] [M] [P]
 1   2   3
```

The Elves just need to know which crate will end up on top of each stack; in this example, the top crates are `C` in stack 1, `M` in stack 2, and `Z` in stack 3, so you should combine these together and give the Elves the message `CMZ`.

**After the rearrangement procedure completes, what crate ends up on top of each stack?**

<details>
<summary><strong>🚧 WIP: See solution 🚧</strong></summary>

To solve this problem, we first need to parse all the lines coming from the input file into a structures that we can actually work with. From the challenge description, there are three different types of lines, each of them described below.

**Lines with crate data**

These lines represent the current structure of a row of stacks, either with or without crates in them. Examples are:

- `[Z] [M] [P]`
- `        [D]`
- `[N] [C]    `

**Lines with command data**

These lines represent the commands we need to apply on the stacks. Examples are:

- `move 1 from 2 to 1`
- `move 3 from 1 to 3`
- `move 2 from 2 to 1`

**Lines with stack numbers**
These lines contain numbers for each stack. An example is `1   2   3`.

To process these lines, we can introduce the concept of a "column". A column is a sequence of three characters, followed by trailing a space. For example, we can say that `[Z] [M] [P]` has three columns: `[Z] `, `[M] ` and `[P]`. Based on this observation, we can collect all the chars of a line into a vector and then split it into chunks of four:

```rust
line.chars().collect::<Vec<char>>().chunks(4)
```

Then, we just need to get rid of the trailing space and convert the chars back into a string. The final code for this part looks like:

```rust
pub fn parse_crate_line(line: &str) -> Vec<String> {
    line.chars()
        .collect::<Vec<char>>()
        .chunks(4)
        .map(|c| c.iter().collect::<String>())
        .map(|s| String::from(s.trim()))
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
    // -- snip --
    #[test]
    fn should_parse_a_crate_line() {
        let line = "[Z] [M] [P]";

        let result = parse_crate_line(&line);

        assert_eq!("[Z]", result.get(0).unwrap());
        assert_eq!("[M]", result.get(1).unwrap());
        assert_eq!("[P]", result.get(2).unwrap());
    }
}
```

Next up, let's handle the liens representing commands. Each command has the same pattern: `move {quantity} from {origin} to {target}`, where `quantity`, `origin` and `target` are numbers. We can capture this pattern in a regular expression, using the `regex` crate, to grab the command parts and store it into a vector:

```rust
pub fn parse_command_line(line: &str) -> Vec<usize> {
    let mut cmd: Vec<usize> = vec![];

    for cap in Regex::new(r"\w+\s\d+\s?").unwrap().captures_iter(line) {
        let cmd_part: Vec<&str> = cap.get(0).unwrap().as_str().trim().split(" ").collect();
        cmd.push(cmd_part.get(1).unwrap().parse::<usize>().unwrap());
    }

    cmd
}

#[cfg(test)]
mod tests {
    // -- snip --
    #[test]
    fn should_parse_a_command_line() {
        let line = "move 1 from 2 to 1";

        let result = parse_command_line(&line);

        assert_eq!(1, *result.get(0).unwrap());
        assert_eq!(2, *result.get(1).unwrap());
        assert_eq!(1, *result.get(2).unwrap());
    }
}
```

We can also introduce a dedicated structure to hold information about a command:

```rust
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
}
```

Gluing all the above together, we can now implement a function that takes a vector of input lines and returns all the parsed rows and the commands:

```rust
pub fn process_input_lines(lines: &Vec<&str>) -> (Vec<Vec<String>>, Vec<CraneMoverCommand>) {
    let mut item_rows: Vec<Vec<String>> = vec![];
    let mut commands: Vec<CraneMoverCommand> = vec![];
    for line in lines {
        if line.contains("[") {
            let line_data = parse_crate_line(line);
            item_rows.push(line_data);
        } else if line.contains("move") {
            let cmd = parse_command_line(line);
            let crane_quantity = *cmd.get(0).unwrap();
            let origin_stack_position = *cmd.get(1).unwrap();
            let target_stack_position = *cmd.get(2).unwrap();

            commands.push(CraneMoverCommand::new(
                crane_quantity,
                origin_stack_position,
                target_stack_position,
            ));
        }
    }

    (item_rows, commands)
}
```

We can now implement a `struct` to represent a crate stack, i.e., a "column":

```rust
pub struct CrateStack {
    items: Vec<String>,
}

impl CrateStack {
    pub fn new(items: Vec<String>) -> CrateStack {
        CrateStack { items }
    }
}
```

We can also implement logic on `CrateStack` so it knows how to create itself from a list of rows:

```rust
impl CrateStack {
    pub fn from_rows(item_rows: &Vec<Vec<String>>) -> Vec<CrateStack> {
        let mut stacks: Vec<CrateStack> = vec![];
        let len = get_number_of_columns_from(item_rows);

        (0..len).into_iter().for_each(|n| {
            let mut stack_items: Vec<String> = Vec::new();
            for row in item_rows {
                match &row.get(n) {
                    Some(v) => {
                        if !v.is_empty() {
                            stack_items.push(v.to_string())
                        }
                    }
                    None => (),
                }
            }

            stacks.push(CrateStack::new(stack_items.clone()));
        });

        stacks
    }
}
```

---

WIP 👇🏽
- `MoveCraneStrategy`:

```rust
pub trait MoveCraneStrategy {
    fn process_move_command(&self, cmd: &CraneMoverCommand, stacks: &mut Vec<CrateStack>);
}
```

- `CraneMoverCommand`:

```rust
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
```

- `CrateStack`:

```rust
pub struct CrateStack {
    items: Vec<String>,
}

impl CrateStack {
    pub fn new(items: Vec<String>) -> CrateStack {
        CrateStack { items }
    }

    pub fn from_rows(item_rows: &Vec<Vec<String>>) -> Vec<CrateStack> {
        let mut stacks: Vec<CrateStack> = vec![];
        let len = get_number_of_columns_from(item_rows);

        (0..len).into_iter().for_each(|n| {
            let mut stack_items: Vec<String> = Vec::new();
            for row in item_rows {
                match &row.get(n) {
                    Some(v) => {
                        if !v.is_empty() {
                            stack_items.push(v.to_string())
                        }
                    }
                    None => (),
                }
            }

            stacks.push(CrateStack::new(stack_items.clone()));
        });

        stacks
    }

    pub fn pop_range(&mut self, range: Range<usize>) -> Vec<String> {
        self.items.drain(range).collect()
    }

    pub fn prepend(&mut self, item: String) {
        self.items.insert(0, item);
    }

    pub fn prepend_many(&mut self, items: Vec<String>) {
        self.items.splice(0..0, items);
    }

    pub fn first(&self) -> &str {
        self.items
            .get(0)
            .expect("expected CrateStack to contain an item")
    }
}
```

- `get_topmost_item_from_each_stack`:

```rust
pub fn get_topmost_item_from_each_stack(stacks: &Vec<CrateStack>) -> String {
    let mut items: Vec<&str> = vec![];
    for c in stacks {
        items.push(c.first());
    }

    items
        .iter()
        .map(|s| s.replace("[", "").replace("]", ""))
        .collect::<Vec<String>>()
        .join("")
}
```

- `CraneMover9000`:

```rust
pub struct CraneMover9000 {
    pub model: String,
}

impl CraneMover9000 {
    pub fn new() -> CraneMover9000 {
        CraneMover9000 {
            model: String::from("Crane Mover 9000"),
        }
    }
}

impl MoveCraneStrategy for CraneMover9000 {
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
        for item in items_to_move {
            target.prepend(item);
        }
    }
}
```

</details>

## Part two: moving more than one crate at once

As you watch the crane operator expertly rearrange the crates, you notice the process isn't following your prediction.

Some mud was covering the writing on the side of the crane, and you quickly wipe it away. The crane isn't a CrateMover 9000 - it's a CrateMover 9001.

The CrateMover 9001 is notable for many new and exciting features: air conditioning, leather seats, an extra cup holder, and the ability to pick up and move multiple crates at once.

Again considering the example above, the crates begin in the same configuration:

```
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3
```

Moving a single crate from stack 2 to stack 1 behaves the same as before:

```
[D]
[N] [C]
[Z] [M] [P]
 1   2   3
```

However, the action of moving three crates from stack 1 to stack 3 means that those three moved crates stay in the same order, resulting in this new configuration:

```
        [D]
        [N]
    [C] [Z]
    [M] [P]
 1   2   3
```

Next, as both crates are moved from stack 2 to stack 1, they retain their order as well:

```
        [D]
        [N]
[C]     [Z]
[M]     [P]
 1   2   3
```

Finally, a single crate is still moved from stack 1 to stack 2, but now it's crate `C` that gets moved:

```
        [D]
        [N]
        [Z]
[M] [C] [P]
 1   2   3
```

In this example, the CrateMover 9001 has put the crates in a totally different order: `MCD`.

Before the rearrangement process finishes, update your simulation so that the Elves know where they should stand to be ready to unload the final supplies. **After the rearrangement procedure completes, what crate ends up on top of each stack?**

<details>
<summary><strong>🚧 WIP: See solution 🚧</strong></summary>

```rust
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
```

</details>
s
