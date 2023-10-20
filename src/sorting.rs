use std::collections::HashMap;

pub fn sort_by_name(files: &mut Vec<(String, u64)>) {
    files.sort_by(|a, b| a.0.cmp(&b.0));
}

pub fn sort_by_size(files: &mut Vec<(String, u64)>) {
    files.sort_by(|a, b| a.1.cmp(&b.1));
}

pub fn sep_files_by_ext(extensions_w_sizes: &mut HashMap<String, u64>, files: &Vec<(String, u64)>) {
    for file in files {
        let file_name = file.0.clone();
        let file_size = file.1.clone();

        let split = file_name.split(".");
        if split.count() == 1 {
            if !extensions_w_sizes.contains_key("no_extension") {
                extensions_w_sizes.insert("no_extension".to_string(), 0);
            }
            let mut value = extensions_w_sizes.get_mut("no_extension").unwrap().clone();
            value += file_size;
            extensions_w_sizes.insert("no_extension".to_string(), value);
            return;
        }

        let extension = file_name.split(".").last().unwrap().to_string();
        if !extensions_w_sizes.contains_key(&extension) {
            extensions_w_sizes.insert(extension.clone(), 0);
        }

        let mut value = extensions_w_sizes.get_mut(&extension).unwrap().clone();
        value += file_size;
        extensions_w_sizes.insert(extension, value);
    }
}

pub fn sort_hashmap_by_value(extensions_w_sizes: &mut HashMap<String, u64>, result: &mut Vec<(String, u64)>) {
    let mut extensions_w_sizes_vec: Vec<(String, u64)> = vec![];
    for (key, value) in extensions_w_sizes {
        extensions_w_sizes_vec.push((key.clone(), value.clone()));
    }
    extensions_w_sizes_vec.sort_by(|a, b| a.1.cmp(&b.1));
    for extension in extensions_w_sizes_vec {
        result.push(extension);
    }
}


