use regex::Regex;

#[derive(Debug, Clone)]
pub struct File {
    pub name: String,
    pub size: usize,
}

#[derive(Debug, Clone)]
pub struct Subdirectory {
    pub name: String,
    pub parent_name: Option<String>,
    pub files: Vec<File>,
    pub directories: Vec<String>,
}

impl File {
    pub fn from_string(s: &str) -> File {
        let parts: Vec<&str> = s.split(" ").collect();
        let size = parts.get(0).unwrap().parse::<usize>().unwrap();
        let name = parts.get(1).unwrap().to_string();

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

    pub fn from_string(s: &str, cwd: &str) -> Subdirectory {
        let parts: Vec<&str> = s.split(" ").collect();
        let mut dir_name = String::from("").to_owned();
        dir_name.push_str(&cwd);

        if cwd != "/" {
            dir_name.push_str("/");
        }

        dir_name.push_str(&parts.get(1).unwrap().to_string());

        let dir = Subdirectory::new(
            dir_name.clone(),
            Some(String::from(cwd.clone())),
            vec![],
            vec![],
        );

        dir
    }
}

pub struct DirRegistry {
    pub items: Vec<Subdirectory>,
}

impl DirRegistry {
    pub fn new(items: Option<Vec<Subdirectory>>) -> DirRegistry {
        DirRegistry {
            items: match items {
                Some(v) => v,
                None => vec![Subdirectory::new(String::from("/"), None, vec![], vec![])],
            },
        }
    }

    pub fn append_dir(&mut self, cwd: &str, dir: Subdirectory) {
        self.items
            .iter_mut()
            .find(|d| d.name == cwd)
            .unwrap()
            .directories
            .push(dir.name.clone());

        self.items.push(dir);
    }

    pub fn append_file(&mut self, cwd: &str, file: File) {
        self.items
            .iter_mut()
            .find(|d| d.name == cwd)
            .unwrap()
            .files
            .push(file);
    }
}

fn compute_new_cwd(target_dir: String, cwd: &str) -> String {
    if target_dir == ".." {
        let mut cwd_parts: Vec<&str> = cwd.split("/").filter(|p| p != &"").collect();
        cwd_parts.pop();

        let mut new_cwd = String::from("/");
        new_cwd.push_str(&cwd_parts.join("/"));

        new_cwd
    } else {
        let mut fully_qualified_target_dir = cwd.clone().to_string();
        if target_dir != "/" {
            if cwd != "/" {
                fully_qualified_target_dir.push_str("/");
            }

            fully_qualified_target_dir.push_str(&target_dir);
        }

        println!("cd {}", fully_qualified_target_dir);
        fully_qualified_target_dir
    }
}

fn process_cmd_line_session(input: &Vec<&str>) -> DirRegistry {
    let mut registry = DirRegistry::new(None);
    let mut cwd = String::from("/");

    for line in input {
        if line.contains("$ cd") {
            let parts: Vec<&str> = line.split(" ").collect();
            let target_dir = parts.get(2).unwrap().to_string();
            cwd = compute_new_cwd(target_dir, &cwd);
        } else if line.contains("$ ls") {
            println!("ls {}", cwd);
        } else if Regex::new(r"\d+\s\w+").unwrap().is_match(line) {
            let file = File::from_string(line);
            println!("- {}", &file.name);
            registry.append_file(&cwd, file.clone());
        } else if line.contains("dir") {
            let dir = Subdirectory::from_string(&line, &cwd);
            println!("- {}", dir.name);
            registry.append_dir(&cwd, dir);
        }
    }

    registry
}

pub fn find_total_sizes_of_directories(input: &Vec<&str>) -> usize {
    let dir_registry = process_cmd_line_session(input);

    let total = dir_registry
        .items
        .iter()
        .map(|d| d.get_size(&dir_registry.items))
        .filter(|s| s <= &100000)
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

    let min = dir_registry
        .items
        .iter()
        .map(|d| d.get_size(&dir_registry.items))
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
