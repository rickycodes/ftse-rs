use crate::constants::{
  BASE_URL,
  FALLERS
};

pub fn format_url(target: String) -> String {
  return format!("{}{}/{}", BASE_URL, target, FALLERS);
}
