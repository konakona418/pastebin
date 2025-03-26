// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn _list_files(path: &str) -> Vec<String> {
    let data_dir = std::path::Path::new(&path);
    if !data_dir.exists() {
        std::fs::create_dir(&data_dir).expect("Failed to create data directory");
    }

    let entries = std::fs::read_dir(data_dir);
    if let Ok(entries) = entries {
        entries
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().is_ok_and(|ft| ft.is_file()))
            .map(|entry| {
                entry
                    .file_name()
                    .into_string()
                    .expect("Failed to convert file name to string")
            })
            .collect()
    } else {
        vec![]
    }
}

#[tauri::command]
fn get_file_list() -> Vec<String> {
    let data_dir_path = format!("{}/data", get_executable_path());
    _list_files(&data_dir_path)
}

#[tauri::command]
fn get_file_list_page(page: usize, page_size: usize) -> Vec<String> {
    let file_list = get_file_list();
    file_list
        .chunks(page_size)
        .nth(page)
        .unwrap_or(&[])
        .to_vec()
}

#[tauri::command]
async fn read_file(file_name: &str) -> Result<Vec<u8>, ()> {
    let data_dir_path = format!("{}/data", get_executable_path());
    let file_path = format!("{}/{}", data_dir_path, file_name);

    if let Ok(file) = std::fs::read(file_path) {
        Ok(file)
    } else {
        Ok(vec![])
    }
}

#[tauri::command]
fn get_directory_list() -> Vec<String> {
    let data_dir_path = format!("{}/data", get_executable_path());
    let data_dir = std::path::Path::new(&data_dir_path);
    if !data_dir.exists() {
        std::fs::create_dir(&data_dir).expect("Failed to create data directory");
    }

    let entries = std::fs::read_dir(data_dir);
    if let Ok(entries) = entries {
        entries
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().is_ok_and(|ft| ft.is_dir()))
            .map(|entry| {
                entry
                    .file_name()
                    .into_string()
                    .expect("Failed to convert file name to string")
            })
            .collect()
    } else {
        vec![]
    }
}

#[tauri::command]
fn get_directory_list_page(page: usize, page_size: usize) -> Vec<String> {
    let directory_list = get_directory_list();
    directory_list
        .chunks(page_size)
        .nth(page)
        .unwrap_or(&[])
        .to_vec()
}

#[tauri::command]
fn get_directory_files(path: &str) -> Vec<String> {
    let path = format!("{}/{}/{}", get_executable_path(), "data", path);
    let data_dir = std::path::Path::new(&path);
    if !data_dir.exists() {
        std::fs::create_dir(&data_dir).expect("Failed to create data directory");
    }

    let entries = std::fs::read_dir(data_dir);
    if let Ok(entries) = entries {
        entries
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().is_ok_and(|ft| ft.is_file()))
            .map(|entry| {
                entry
                    .file_name()
                    .into_string()
                    .expect("Failed to convert file name to string")
            })
            .collect()
    } else {
        vec![]
    }
}

#[tauri::command]
fn get_directory_files_page(dir: &str, page: usize, page_size: usize) -> Vec<String> {
    let directory_files = get_directory_files(dir);
    directory_files
        .chunks(page_size)
        .nth(page)
        .unwrap_or(&[])
        .to_vec()
}

#[tauri::command]
async fn read_directory_file(dir: &str, file_name: &str) -> Result<Vec<u8>, ()> {
    let path = format!("{}/{}/{}/{}", get_executable_path(), "data", dir, file_name);
    if let Ok(file) = std::fs::read(path) {
        Ok(file)
    } else {
        Ok(vec![])
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
        .invoke_handler(tauri::generate_handler![
            greet,
            get_file_list,
            read_file,
            get_file_list_page,
            get_directory_list,
            get_directory_list_page,
            get_directory_files,
            get_directory_files_page,
            read_directory_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
