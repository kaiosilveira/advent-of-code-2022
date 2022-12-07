use super::file::File;

#[derive(Debug, Clone)]
pub struct Subdirectory {
    pub name: String,
    pub parent_name: Option<String>,
    pub files: Vec<File>,
    pub directories: Vec<String>,
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
