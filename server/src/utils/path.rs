use std::path::PathBuf;
use super::error::AppError;

pub fn secure_join(root: &PathBuf, unsafe_path: &PathBuf) -> Result<PathBuf, AppError> {
  if unsafe_path.has_root() {
    return Err(AppError::new("path error: path has root"));
  }
  if let Some(s) = unsafe_path.to_str() {
    if s.contains("..") {
      return Err(AppError::new("path error"));
    }
  }
  let new_path = root.join(unsafe_path);
  Ok(new_path)
}

#[allow(unused)]
pub fn secure_join_symlink(root: &PathBuf, unsafe_path: &PathBuf) -> Result<PathBuf, AppError> {
  if unsafe_path.has_root() {
    return Err(AppError::new("path error"));
  }
  let new_path = root.join(unsafe_path);
  let t1 = new_path.canonicalize()?;
  let t2 = root.canonicalize()?;

  let nc = t1.components();
  let rc = t2.components();
  
  if nc.into_iter().count() < rc.into_iter().count() {
    return Err(AppError::new("path error"));
  }
  Ok(new_path)
}
