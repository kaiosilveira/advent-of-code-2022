use super::{directory::Subdirectory, file::File};

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

  pub fn get_dir_sizes_below_threshold(&self, threshold: &usize) -> Vec<usize> {
      self.items
          .iter()
          .map(|d| d.get_size(&self.items))
          .filter(|s| s <= threshold)
          .collect::<Vec<usize>>()
  }

  pub fn get_dir_sizes_above_threshold(&self, threshold: &usize) -> Vec<usize> {
      self.items
          .iter()
          .map(|d| d.get_size(&self.items))
          .filter(|s| s > threshold)
          .collect::<Vec<usize>>()
  }
}
