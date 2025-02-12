use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FileKv {
  pub abs_path: String,
  pub created_at: u64,
  pub mod_at: u64,
  pub size: u64,
  pub is_dir: bool,
  // pub is_symbol: bool,
}

#[test]
fn t1() {
  let file = FileKv {
    abs_path: String::from("jack"),
    created_at: 123,
    mod_at: 214,
    size: 52,
    // is_symbol: false,
    is_dir: false,
  };
  let result = serde_json::to_string(&file);
  println!("{:?}", result);
}
