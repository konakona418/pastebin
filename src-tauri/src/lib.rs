// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_file_list() -> Vec<String> {
    let data_dir_path = format!("{}/data", get_executable_path());
    let data_dir = std::path::Path::new(&data_dir_path);
    if !data_dir.exists() {
        std::fs::create_dir(&data_dir).expect("Failed to create data directory");
    }

    let entries = std::fs::read_dir(data_dir);
    if let Ok(entries) = entries {
        entries
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.file_name().into_string().unwrap())
            .collect()
    } else {
        vec![]
    }
}

#[tauri::command]
fn read_file(file_name: &str) -> Vec<u8> {
    let data_dir_path = format!("{}/data", get_executable_path());
    let file_path = format!("{}/{}", data_dir_path, file_name);

    if let Ok(file) = std::fs::read(file_path) {
        file
    } else {
        vec![]
    }
}

fn is_directory_exists(path: &str) -> bool {
    std::fs::metadata(path).map(|m| m.is_dir()).unwrap_or(false)
}

fn get_executable_path() -> String {
    let path = std::env::current_exe().expect("Failed to get executable path");
    let path = path.parent().expect("Failed to get parent directory");
    if path.is_absolute() {
        return path.to_str().unwrap().to_string();
    } else {
        let cwd = std::env::current_dir().expect("Failed to get current directory");
        return format!("{}/{}", cwd.to_str().unwrap(), path.display());
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let executable_path = get_executable_path();
    let concat = format!("{}/data", executable_path);
    if !is_directory_exists(&concat) {
        std::fs::create_dir(&concat).expect("Failed to create directory");
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_file_list, read_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
