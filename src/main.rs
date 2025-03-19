use std::env;
use std::io::Write;
use std::path::{ PathBuf, Path };
use std::process::Stdio;
use std::io::Error;
use std::fs::{self, metadata};

fn get_system_config_dir_path() -> PathBuf {
    match env::var("XDG_CONFIG_HOME") {
        Ok(dir_path_str) => PathBuf::from(dir_path_str),
        Err(_) => {
            let home: String = env::var("HOME").expect("HOME system variable is not defined, cannot proceed"); 
            PathBuf::from(home).join(".config")           
        }
    }
}

fn contains_init_file(dir_path: &PathBuf) -> bool {
    let init_file: PathBuf = if dir_path.join("init.lua").exists() { 
        dir_path.join("init.lua") 
    } else if dir_path.join("init.vim").exists() { 
        dir_path.join("init.vim") 
    } else { return false };

    !metadata(init_file).unwrap().is_dir()
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().skip(1).collect();

    let system_config_dir_path_buf = get_system_config_dir_path();
    let system_config_dir: &Path = system_config_dir_path_buf.as_path();
    let root_nvim_config_dir: PathBuf = PathBuf::new().join(system_config_dir).join("nvim");
    
    let root_nvim_dir_contents = fs::read_dir(root_nvim_config_dir.as_path())?;

    let mut configs_paths: Vec<PathBuf> = Vec::new();
    
    if contains_init_file(&root_nvim_config_dir) { 
        configs_paths.push(root_nvim_config_dir.clone().strip_prefix(system_config_dir).unwrap().to_path_buf()) 
    }

    for content_or_err in root_nvim_dir_contents {
        let content = content_or_err.as_ref().unwrap();
        let name: String = String::from(content.file_name().to_str().unwrap());
        if name.starts_with(".") { continue };

        let path = content.path();
        let metadata = fs::metadata(&path).expect("Error while fetching metadata of {content}");
        
        if !metadata.is_dir() { continue }
        if contains_init_file(&path) {
            configs_paths.push(path.strip_prefix(system_config_dir).unwrap().to_path_buf());
        }
    }

    let mut fzf = std::process::Command::new("fzf")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .arg("--border")
        .spawn()
        .unwrap();
    
    let dir_names: Vec<&str> = configs_paths.iter().map(|path| path.to_str().unwrap()).collect();

    let stdin = fzf.stdin.as_mut().unwrap();
    stdin.write_all(dir_names.join("\n").as_bytes())?;
    
    let output = fzf.wait_with_output()?;

    let selected_config_dir_raw = String::from_utf8_lossy(&output.stdout);
    let selected_config_dir_trimmed = selected_config_dir_raw.trim();
    
    let _ = std::process::Command::new("nvim")
        .env("NVIM_APPNAME", selected_config_dir_trimmed)
        .args(args)
        .spawn()?
        .wait();

    Ok(())
}
