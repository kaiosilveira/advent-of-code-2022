mod file_system;
use file_system::{directory::*, file::*, registry::*, utils::compute_new_cwd_from};

use regex::Regex;

fn process_cmd_line_session(input: &Vec<&str>) -> DirRegistry {
    let mut registry = DirRegistry::new(None);
    let mut cwd = String::from("/");

    let file_name_pattern_regex = Regex::new(r"\d+\s\w+").unwrap();
    for line in input {
        match line {
            x if x.contains("$ cd") => cwd = compute_new_cwd_from(line, &cwd),
            x if x.contains("dir") => {
                let dir = Subdirectory::from_string(&line, &cwd);
                println!("- {}", dir.name);
                registry.append_dir(&cwd, dir);
            }
            x if file_name_pattern_regex.is_match(x) => {
                let file = File::from_string(line);
                println!("- {}", &file.name);
                registry.append_file(&cwd, file.clone());
            }
            _ => (),
        }
    }

    registry
}

pub fn find_total_sizes_of_directories(input: &Vec<&str>) -> usize {
    let dir_registry = process_cmd_line_session(input);

    let total = dir_registry
        .get_dir_sizes_below_threshold(&100000)
        .iter()
        .fold(0, |a, b| a + b);

    total
}

pub fn find_dir_to_delete(input: &Vec<&str>) -> usize {
    let dir_registry = process_cmd_line_session(input);

    let disk_space = 70000000;
    let required_space_for_update = 30000000;
    let consumed_space = dir_registry.items[0].get_size(&dir_registry.items);
    let available_space = disk_space - consumed_space;
    let needed_space = required_space_for_update - available_space;

    *dir_registry
        .get_dir_sizes_above_threshold(&needed_space)
        .iter()
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_harness_for_part_one() {
        let input = vec![
            "$ cd /",
            "$ ls",
            "dir a",
            "14848514 b.txt",
            "8504156 c.dat",
            "dir d",
            "$ cd a",
            "$ ls",
            "dir e",
            "29116 f",
            "2557 g",
            "62596 h.lst",
            "$ cd e",
            "$ ls",
            "584 i",
            "$ cd ..",
            "$ cd ..",
            "$ cd d",
            "$ ls",
            "4060174 j",
            "8033020 d.log",
            "5626152 d.ext",
            "7214296 k",
        ];

        assert_eq!(95437, find_total_sizes_of_directories(&input));
    }

    #[test]
    fn test_harness_for_part_two() {
        let input = vec![
            "$ cd /",
            "$ ls",
            "dir a",
            "14848514 b.txt",
            "8504156 c.dat",
            "dir d",
            "$ cd a",
            "$ ls",
            "dir e",
            "29116 f",
            "2557 g",
            "62596 h.lst",
            "$ cd e",
            "$ ls",
            "584 i",
            "$ cd ..",
            "$ cd ..",
            "$ cd d",
            "$ ls",
            "4060174 j",
            "8033020 d.log",
            "5626152 d.ext",
            "7214296 k",
        ];

        assert_eq!(24933642, find_dir_to_delete(&input));
    }
}
