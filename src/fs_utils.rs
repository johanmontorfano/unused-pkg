use std::{fs::read_to_string, path::PathBuf};
use walkdir::WalkDir;

const IGNORED_FOLDERS: [&str; 6] = [
    "target",
    "node_modules",
    ".git",
    ".idea",
    ".next",
    "build"
];
const IGNORED_FILES: [&str; 5] = [
    "package.json",
    "package-lock.json",
    "pnpm-lock.yaml",
    "tsconfig.json",
    "Cargo.toml"
];

/// Checks if a given file at a given path exists.
pub fn try_exists_at_path(path: &PathBuf, file_name: &str) -> bool {
    let mut path = path.clone();
    
    path.push(file_name);
    path.exists()
}

pub fn contains_none_of(string: &str, items: Vec<&str>) -> bool {
    items.iter().fold(true, |found, item| {
        if string.contains(item) {
            return false;
        }
        found
    })
}

/// Recursively list each file from a directory entry. 
/// Files in IGNORED_FOLDERS are automatically ignored. 
/// Files in IGNORED_FILES are automatically ignored.
pub fn recursive_listing(source: &PathBuf) -> Vec<String> {
    let mut files_content = Vec::new();
    for entry in WalkDir::new(source).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        let str_path = path.to_string_lossy().to_string();

        if contains_none_of(&str_path, IGNORED_FOLDERS.into()) &&
            contains_none_of(&str_path, IGNORED_FILES.into()) &&
            path.is_file() {
            if let Ok(content) = read_to_string(path) {
                files_content.push(content);
            }
        }
    }
    files_content
}
