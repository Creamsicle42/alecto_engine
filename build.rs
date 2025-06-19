#![allow(unused)]
use std::{
    env,
    fs::{self, File, read_dir},
    io::Write,
    path::{Path, PathBuf},
};

use zip::{ZipWriter, write::SimpleFileOptions};

fn main() {
    //let out_dir = env::var_os("CARGO_BIN_NAME").unwrap();
    //let dest_path = Path::new(&out_dir).parent().unwrap().join("assets.zip");
    let dest_path = Path::new("target/debug/assets.zip");
    let out_file = File::create(dest_path).unwrap();
    let zip_options = SimpleFileOptions::default();
    let mut archive = ZipWriter::new(out_file);

    for arch_file in get_files_recursive("assets/").iter() {
        let reduced_path = arch_file.strip_prefix("assets/").unwrap();
        println!("Archiving {:?}", arch_file);
        archive.start_file(reduced_path.as_os_str().to_str().unwrap(), zip_options);
        let _ = archive.write(fs::read(arch_file).unwrap().as_slice());
    }

    archive.finish();
    println!("cargo::rerun-if-changed=assets/*");
}

fn get_files_recursive<P: AsRef<Path>>(path: P) -> Vec<PathBuf> {
    let mut out: Vec<_> = vec![];

    for entry in read_dir(path).unwrap().filter_map(|x| x.ok()) {
        let path = entry.path();
        if path.is_file() {
            out.push(path);
        } else if path.is_dir() {
            out.append(&mut get_files_recursive(path));
        }
    }

    out
}
