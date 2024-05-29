use walkdir::WalkDir;

pub fn get_all_files() -> Vec<String> {
    let mut file_paths = Vec::new();
    for entry in WalkDir::new(".").into_iter().filter_map(Result::ok) {
        if entry.file_type().is_file() {
            let path_str = entry
                .path()
                .strip_prefix(".")
                .unwrap_or_else(|_| entry.path())
                .display()
                .to_string();
            file_paths.push(path_str);
        }
    }
    file_paths
}

pub fn split_files(files: Vec<String>) -> Vec<Vec<String>> {
    let paths: Vec<Vec<String>> = files
        .iter()
        .map(|f| f.split("/").map(|s| s.to_string()).collect())
        .collect();

    paths
}

pub fn filter_pages(files: &Vec<Vec<String>>) -> Option<Vec<Vec<String>>> {
    let mut pages: Vec<Vec<String>> = Vec::new();
    for dir in files {
        if dir[0] != "pages" {
            continue;
        }

        pages.push(dir.to_vec());
    }
    Some(pages)
}
