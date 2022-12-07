#[derive(Debug, Clone)]
pub struct File {
    pub name: String,
    pub size: usize,
}

impl File {
  pub fn from_string(s: &str) -> File {
      let parts: Vec<&str> = s.split(" ").collect();
      let size = parts.get(0).unwrap().parse::<usize>().unwrap();
      let name = parts.get(1).unwrap().to_string();

      File { name, size }
  }
}
