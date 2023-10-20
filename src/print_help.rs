pub fn print_help() {
    println!("Usage: ./file_sorter <path>");
    println!("Example: ./file_sorter /home/user/Documents (all files in Documents and subdirectories)");
    println!("You can also use wildcards");
    println!("Example: ./file_sorter /home/user/Documents/*.txt (only txt files)");
    println!("Example: ./file_sorter /home/user/Documents/*.* (all files in Documents)");
    println!("Example: ./file_sorter /home/user/Documents/**.* (all files in Documents and subdirectories)");
}