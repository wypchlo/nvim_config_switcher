use std::env;
use std::io::Write;
use std::path::{ PathBuf, Path };
use std::process::Stdio;
use std::io::Error;
use std::fs;

fn get_system_config_dir_path() -> PathBuf {
    match env::var("XDG_CONFIG_HOME") {
        Ok(dir_path_str) => PathBuf::from(dir_path_str),
        Err(_) => {
            let home: String = env::var("HOME").expect("HOME system variable is not defined, cannot proceed"); 
            PathBuf::from(home).join(".config")           
        }
    }
}

fn main() -> Result<(), Error> {
    let system_config_dir_path_buf = get_system_config_dir_path();
    let system_config_dir: &Path = system_config_dir_path_buf.as_path();
    let root_nvim_config_dir: PathBuf = PathBuf::new().join(system_config_dir).join("nvim");
    
    let root_nvim_dir_contents = fs::read_dir(root_nvim_config_dir.as_path())?;

    let mut configs_paths: Vec<PathBuf> = Vec::new();
    
    for content_or_err in root_nvim_dir_contents {
        let content = content_or_err.as_ref().unwrap();
        let name: String = String::from(content.file_name().to_str().unwrap());
        if name.starts_with(".") { continue };
        if name == "init.vim" || name == "init.lua" { 
            configs_paths.push(root_nvim_config_dir.clone().strip_prefix(system_config_dir).unwrap().to_path_buf()) 
        }

        let path = content.path();
        let metadata = fs::metadata(&path).expect("Error while fetching metadata of {content}");
        
        if !metadata.is_dir() { continue }
        if path.join("init.lua").exists() || path.join("init.vim").exists() {
            configs_paths.push(path.strip_prefix(system_config_dir).unwrap().to_path_buf())
        }
    }

    let mut fzf = std::process::Command::new("fzf")
        .stdin(Stdio::piped())
        .arg("--border")
        .spawn()
        .unwrap();
    
    let dir_names: Vec<&str> = configs_paths.iter().map(|path| path.to_str().unwrap()).collect();

    let stdin = fzf.stdin.as_mut().unwrap();
    stdin.write_all(dir_names.join("\n").as_bytes())?;

    fzf.wait_with_output()?;

    Ok(())
}
