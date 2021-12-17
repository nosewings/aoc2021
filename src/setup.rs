use std::fs;
use std::os::unix;
use std::path::PathBuf;
use std::str::FromStr;

fn main() {
    for entry in fs::read_dir("hooks").unwrap() {
        let basename = entry.unwrap().file_name();
        let basename = basename.to_str().unwrap();
        let from: PathBuf = ["..", "..", "hooks", basename].iter().collect();
        let mut to = PathBuf::from_str(".git/hooks").unwrap();
        to.push(basename);
        if to.exists() {
            continue;
        }
        unix::fs::symlink(from, to).unwrap();
    }
}
