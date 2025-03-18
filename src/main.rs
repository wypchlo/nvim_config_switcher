use std::env;
use std::io::Write;
use std::path::PathBuf;
use std::process::Stdio;
use std::io::Error;
use std::fs;

fn get_config_dir_path() -> PathBuf {
    let xdg_config_home = match env::var("XDG_CONFIG_HOME") {
        Ok(dir) => Some(dir),
        Err(_) => None
    };
    
    if let Some(dir) = xdg_config_home { 
        return PathBuf::from(dir) 
    }

    let home: String = env::var("HOME").expect("HOME system variable is not defined, cannot proceed");

    [home.as_str(), ".config"].iter().collect()
}

fn main() -> Result<(), Error> {
    let config_dir: PathBuf = get_config_dir_path();
    let nvim_config_dir: PathBuf = config_dir.join("nvim");
    
    let nvim_dir_contents = fs::read_dir(nvim_config_dir.as_path())?;

    let mut configs_paths: Vec<PathBuf> = Vec::new();
    
    for content_or_err in nvim_dir_contents {
        let content = content_or_err.as_ref().unwrap();
        let name: String = String::from(content.file_name().to_str().unwrap());
        if name.starts_with(".") { continue };
        if name == "init.vim" || name == "init.lua" { configs_paths.push(nvim_config_dir.clone()) }

        let path = content.path();
        let metadata = fs::metadata(&path).expect("Error while fetching metadata of {content}");
        
        if !metadata.is_dir() { continue }
        if path.join("init.lua").exists() || path.join("init.vim").exists() {
            configs_paths.push(path)
        }
    }

    let mut fzf = std::process::Command::new("fzf")
        .stdin(Stdio::piped())
        .arg("--border")
        .spawn()
        .unwrap();
    
    let folder_names: Vec<&str> = configs_paths.iter().map(|path| path.file_name().unwrap().to_str().unwrap()).collect();

    let stdin = fzf.stdin.as_mut().unwrap();
    stdin.write_all(folder_names.join("\n").as_bytes())?;

    fzf.wait_with_output()?;

    Ok(())
}
