use std::{env, path::{Path, PathBuf}};

const DEFAULT_CONFIG_FILE_NAME:&str = "farm.config";

/// 找到当前根目录下的config配置文件路径
pub fn find_config_path()->Result<PathBuf, &'static str>{
  let  root: PathBuf = find_cwd();
  let ext = ["js","ts","mjs"];

  for &item in ext.iter(){
    // 处理不同的文件名
    let file_name = format!("{}.{}",DEFAULT_CONFIG_FILE_NAME,item);
    let mut new_root = root.clone();
    new_root.push(file_name);
   
    if new_root.is_file()==true {
     return Ok(new_root)
    }
  }
  Err("not found config file")
}


/// 获取当前环境的运行根目录,类似cwd()
pub fn find_cwd()->PathBuf{
  let root = env::current_dir();
  // 获取成功就用值,否则给个默认.
  root.unwrap_or(Path::new(".").to_path_buf())
}
