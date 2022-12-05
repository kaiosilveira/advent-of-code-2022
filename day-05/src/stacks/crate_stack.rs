use std::ops::Range;

use crate::get_number_of_columns_from;

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
