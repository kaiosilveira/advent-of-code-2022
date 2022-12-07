use regex::Regex;

#[derive(Debug, Clone)]
pub struct File {
    pub name: String,
    pub size: usize,
}

#[derive(Debug)]
pub struct Subdirectory {
    pub name: String,
    pub parent_name: Option<String>,
    pub files: Vec<File>,
    pub directories: Vec<String>,
}

impl File {
    pub fn new(name: String, size: usize) -> File {
        File { name, size }
    }
}

impl Subdirectory {
    pub fn new(
        name: String,
        parent_name: Option<String>,
        files: Vec<File>,
        directories: Vec<String>,
    ) -> Subdirectory {
        Subdirectory {
            name,
            parent_name,
            files,
            directories,
        }
    }

    pub fn get_size(&self, dir_registry: &Vec<Subdirectory>) -> usize {
        self.files.iter().map(|f| f.size).fold(0, |a, b| a + b)
            + dir_registry
                .iter()
                .filter(|d| self.directories.contains(&d.name))
                .map(|d| d.get_size(dir_registry))
                .fold(0, |a, b| a + b)
    }
}

fn process_cmd_line_session(input: &Vec<&str>) -> Vec<Subdirectory> {
    let mut dir_registry: Vec<Subdirectory> =
        vec![Subdirectory::new(String::from("/"), None, vec![], vec![])];

    let mut cwd = String::from("/");

    for line in input {
        let parts: Vec<&str> = line.split(" ").collect();
        if line.contains("$ cd") {
            let target_dir = parts.get(2).unwrap().to_string();
            println!("navigating to {} from {}", target_dir, cwd);

            if target_dir == ".." {
                cwd = dir_registry
                    .iter_mut()
                    .find(|d| d.name == cwd)
                    .unwrap()
                    .parent_name
                    .as_ref()
                    .unwrap()
                    .to_string();
            } else {
                cwd = dir_registry
                    .iter_mut()
                    .find(|d| d.name == target_dir)
                    .unwrap()
                    .name
                    .to_string();
            }
        } else if line.contains("$ ls") {
            println!("Running ls command on {}...", cwd);
        } else if Regex::new(r"\d+\s\w+").unwrap().is_match(line) {
            let file_size = parts.get(0).unwrap().parse::<usize>().unwrap();
            let file_name = parts.get(1).unwrap().to_string();

            let file = File::new(file_name, file_size);

            println!("Found file: {:?}", file);

            dir_registry
                .iter_mut()
                .find(|d| d.name == cwd)
                .unwrap()
                .files
                .push(file);
        } else if line.contains("dir") {
            let dir_name = parts.get(1).unwrap().to_string();
            let dir = Subdirectory::new(
                dir_name.clone(),
                Some(String::from(cwd.clone())),
                vec![],
                vec![],
            );
            println!("Found dir: {:?}", dir);

            dir_registry
                .iter_mut()
                .find(|d| d.name == cwd)
                .unwrap()
                .directories
                .push(dir_name.clone());

            dir_registry.push(dir);
        }
    }

    dir_registry
}

pub fn find_total_sizes_of_directories(input: &Vec<&str>) -> usize {
    let dir_registry = process_cmd_line_session(input);

    let total = dir_registry
        .iter()
        .map(|d| d.get_size(&dir_registry))
        .filter(|s| s <= &100000)
        .fold(0, |a, b| a + b);

    total
}

pub fn find_dir_to_delete(input: &Vec<&str>) -> usize {
    let dir_registry = process_cmd_line_session(input);

    let disk_space = 70000000;
    let required_space_for_update = 30000000;
    let consumed_space = dir_registry[0].get_size(&dir_registry);
    let available_space = disk_space - consumed_space;
    let needed_space = required_space_for_update - available_space;

    let min = dir_registry
        .iter()
        .map(|d| d.get_size(&dir_registry))
        .filter(|s| s > &needed_space)
        .min()
        .unwrap();

    min
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
