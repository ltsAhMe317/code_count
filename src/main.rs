use std::fmt::Display;
use std::fs;
use std::io::stdout;
use std::path::{Path, PathBuf};

pub fn find_all_rs(path: impl AsRef<Path>) -> Vec<(PathBuf, usize)> {
    let path = path.as_ref();
    let mut vec = Vec::new();
    if path.is_dir() {
        for file in fs::read_dir(path).unwrap() {
            let file = file.unwrap();
            vec.extend(find_all_rs(file.path()));
        }
    } else {
        let file_name = path.file_name().unwrap().to_str().unwrap();
        if let Some(index) = file_name.rfind('.') {
            let suffix = &file_name[index + 1..];
            // dbg!(suffix);
            if suffix == "rs" {
                vec.push((
                    path.to_path_buf(),
                    fs::read_to_string(path).unwrap().trim().len(),
                ));
            }
        }
    }
    vec
}
fn main() {
    let mut args = std::env::args();
    args.next();
    let path = if let Some(str) = args.next() {
        str.trim().to_string()
    } else {
        "./src/".to_string()
    };
    let mut find = find_all_rs(path);
    let mut all_count = 0;
    for (_, size) in find.iter() {
        all_count += size;
    }
    find.sort_by(|(_, size1), (_, size2)| size1.cmp(size2));
    for (path, size) in find.iter() {
        println!("{}:{}", path.file_name().unwrap().to_str().unwrap(), size);
    }
}
