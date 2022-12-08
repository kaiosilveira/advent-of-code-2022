fn build_grid_from(input: &Vec<&str>) -> Vec<Vec<usize>> {
    let mut grid: Vec<Vec<usize>> = vec![];

    for line in input {
        let trees: Vec<usize> = line
            .split("")
            .filter(|s| s != &"")
            .map(|t| t.parse::<usize>().unwrap())
            .collect();

        grid.push(trees);
    }

    grid
}

pub fn count_visible_trees_from(tree: &usize, trees: &Vec<&usize>) -> usize {
    let no_trees_in_front = trees.len() == 0;
    if no_trees_in_front {
        return 0;
    }

    let all_other_trees_are_smaller = trees.iter().all(|t| t < &tree);
    if all_other_trees_are_smaller {
        return trees.len();
    }

    // This case is trickier, as we need to count all the smaller trees directly in front of the
    // given tree, but also compute the first occurrence of a tree that's either of the same
    // height or taller than it
    let mut visibility = 0;
    for t in trees {
        visibility += 1;
        if t >= &tree {
            break;
        }
    }

    visibility
}

pub fn get_scenic_score_for_side(
    idx: &usize,
    list_size: &usize,
    tree: &usize,
    other_trees: &Vec<&usize>,
) -> usize {
    match idx {
        0 => 0,                          // tree is at the first position of the list
        x if x == &(list_size - 1) => 0, // tree is at the last position of the list
        _ => count_visible_trees_from(tree, other_trees),
    }
}

pub fn get_tree_grid_info(input: &Vec<&str>) -> (usize, usize) {
    let grid = build_grid_from(&input);

    let mut number_of_visible_trees = 0;
    let mut highest_scenic_score = 0;

    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, tree) in row.iter().enumerate() {
            // The list of trees on the left side should be reverted so we can have the tree's perspective
            let trees_on_the_left: Vec<&usize> = row[..col_idx].iter().rev().map(|t| t).collect();
            let trees_on_the_right: Vec<&usize> = row[col_idx + 1..].iter().map(|t| t).collect();
            let trees_on_top: Vec<&usize> = grid[..row_idx]
                .iter()
                .rev() // also reversing this list so we can have the tree's perspective
                .map(|r| r.get(col_idx).unwrap())
                .collect();

            let trees_on_bottom: Vec<&usize> = grid[row_idx + 1..]
                .iter()
                .map(|r| r.get(col_idx).unwrap())
                .collect();

            let scenic_score_at_left =
                get_scenic_score_for_side(&col_idx, &row.len(), &tree, &trees_on_the_left);

            let scenic_score_at_right =
                get_scenic_score_for_side(&row_idx, &row.len(), &tree, &trees_on_the_right);

            let scenic_score_top =
                get_scenic_score_for_side(&row_idx, &grid.len(), &tree, &trees_on_top);

            let scenic_score_bottom =
                get_scenic_score_for_side(&row_idx, &grid.len(), &tree, &trees_on_bottom);

            let scenic_score = scenic_score_at_left
                * scenic_score_at_right
                * scenic_score_top
                * scenic_score_bottom;

            if scenic_score > highest_scenic_score {
                highest_scenic_score = scenic_score;
            }

            let is_visible = trees_on_the_left.iter().all(|t| t < &tree)
                || trees_on_the_right.iter().all(|t| t < &tree)
                || trees_on_top.iter().all(|t| t < &tree)
                || trees_on_bottom.iter().all(|t| t < &tree);

            if is_visible {
                number_of_visible_trees += 1;
            }
        }
    }

    (highest_scenic_score, number_of_visible_trees)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_tree_grid_info() {
        let input = vec!["30373", "25512", "65332", "33549", "35390"];
        let (highest_scenic_score, number_of_visible_trees) = get_tree_grid_info(&input);
        assert_eq!(21, number_of_visible_trees);
        assert_eq!(8, highest_scenic_score);
    }
}
