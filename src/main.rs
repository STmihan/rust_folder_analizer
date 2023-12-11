mod sorting;
mod print_help;

extern crate glob;

use std::collections::HashMap;
use std::env::args;
use crate::print_help::print_help;
use crate::sorting::{sep_files_by_ext, sort_by_name, sort_by_size, sort_hashmap_by_value};

fn get_all_files_in_path(path: String, files: &mut Vec<(String, u64)>) {
    let path = format!("{}/**/*", path);
    let result = glob::glob(path.as_str()).unwrap();
    let mut counter = 0;
    for entry in result {
        println!("processing file {}", counter);
        counter += 1;
        let entity = entry.unwrap();
        if entity.is_dir() {
            continue;
        }
        let file_name = entity.file_name().unwrap().to_str().unwrap().to_string();
        let file_size = entity.metadata().unwrap().len();
        files.push((file_name, file_size));
    }
}

fn add_result(result: &mut String, file: (String, u64)) {
    let raw = file.1.clone();
    let kb = file.1 as f64 / 1024.0;
    let mb = file.1 as f64 / 1024.0 / 1024.0;
    let mut formatted;
    formatted = format!("{:.2} MB", mb);
    if mb < 1.0 {
        formatted = format!("{:.2} KB", kb);
    }
    if kb < 1.0 {
        formatted = format!("{} B", raw);
    }
    if mb > 1024.0 {
        let gb = mb / 2048.0;
        formatted = format!("{:.2} GB", gb);
    }

    *result += &*format!("{}, {}, {}\n", file.0, formatted, raw);
}

fn select_action() -> u32 {
    println!("1. Sort by name");
    println!("2. Sort by size");
    println!("3. Sort by size by extension");
    println!("--------------------------------");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    let input: u32 = input.parse().unwrap();
    return input;
}

fn main() {
    if args().len() < 2 {
        println!("Please provide a path to search");
        return;
    }
    let arg1 = args().nth(1).unwrap();
    if arg1 == "-h" || arg1 == "--help" || arg1 == "-help" || arg1 == "help" {
        print_help();
        return;
    }
    let path = arg1;
    let mut result = String::new();

    let mut files: Vec<(String, u64)> = vec![];

    if std::path::Path::new(&path).exists() == false {
        println!("Path does not exist");
        return;
    }
    if std::path::Path::new(&path).is_dir() == false {
        let entity = glob::glob(path.as_str()).unwrap().next().unwrap().unwrap();
        println!("file name: {}", entity.file_name().unwrap().to_str().unwrap());
        println!("file size: {}", entity.metadata().unwrap().len());
        println!("file path: {}", entity.to_str().unwrap());
        println!("file extension: {}", entity.extension().unwrap().to_str().unwrap());
        return;
    }




    get_all_files_in_path(path, &mut files);
    println!("We found {} files", files.len());

    let input = select_action();

    match input {
        1 => {
            sort_by_name(&mut files);
            for file in files {
                add_result(&mut result, file);
            }
        }
        2 => {
            sort_by_size(&mut files);
            for file in files {
                add_result(&mut result, file);
            }
        }
        3 => {
            let mut extensions_w_sizes: HashMap<String, u64> = HashMap::new();
            sep_files_by_ext(&mut extensions_w_sizes, &files);
            let mut sorted: Vec<(String, u64)> = vec![];
            sort_hashmap_by_value(&mut extensions_w_sizes, &mut sorted);
            for file in sorted {
                add_result(&mut result, file);
            }
        }
        _ => {}
    }

    println!("{}", result);
    std::fs::write("result.csv", result).unwrap();
}
