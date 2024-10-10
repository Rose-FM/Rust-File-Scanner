mod lib;
use lib::scan_home_directory;

fn main() {
    match scan_home_directory() {
        Ok(directories) => {
            for dir in directories {
                println!("{:?}", dir);
            }
        }
        Err(e) => eprintln!("Error scanning home directory: {}", e),
    }
}