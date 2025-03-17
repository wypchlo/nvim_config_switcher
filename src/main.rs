use std::env;
use std::path::PathBuf;

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

fn main() {
    use std::fs;
    use std::fs::metadata;

    let config_dir: PathBuf = get_config_dir_path();
    let nvim_dir: PathBuf = config_dir.join("nvim");
    
    let configs = fs::read_dir(nvim_dir.to_str().unwrap()).unwrap();
    for config in configs {
        let config_path_string = String::from(config.unwrap().path().to_str().unwrap());
        let md = metadata(&config_path_string).unwrap();
        if !md.is_dir() { continue }

        println!("{}", config_path_string);
    }
}
