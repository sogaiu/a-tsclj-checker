use glob::glob;
use rayon::prelude::*;
use std::env;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{SystemTime};
use tree_sitter::{Parser, Language};
use std::io::Read;

extern "C" { fn tree_sitter_clojure() -> Language; }

fn parse_from_bytes(bytes: &[u8]) -> bool {
    let language: Language = unsafe { tree_sitter_clojure() };
    let mut parser = Parser::new();
    parser.set_language(language).unwrap();
    let tree = parser.parse(&bytes, None).unwrap();
    let root_node = tree.root_node();
    !root_node.has_error()
}

fn parse_from_file_path(path: &Path, file_count: &AtomicUsize) {
    let language: Language = unsafe { tree_sitter_clojure() };
    let mut parser = Parser::new();
    parser.set_language(language).unwrap();
    let contents = std::fs::read_to_string(path);
    match contents {
        Ok(contents) => {
            let bytes = contents.as_bytes();
            let tree = parser.parse(&bytes, None).unwrap();
            let root_node = tree.root_node();
            file_count.fetch_add(1, Ordering::Relaxed);
            if root_node.has_error() {
                println!("{}", path.to_string_lossy());
            }
        },
        // Error when reading non-UTF8:
        _ => {
            eprintln!("non-UTF8: {}", path.to_string_lossy());
        }
    }
}

// references:
// https://github.com/mvdnes/zip-rs/blob/master/examples/extract.rs
// http://siciarz.net/24-days-rust-zip-and-lzma-compression/
fn parse_from_zipfile_path(path: &Path, file_count: &AtomicUsize) {
    let file = std::fs::File::open(&path).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();
    let range = 0..archive.len();
    let mut f_and_c: Vec<(String, String)> = vec![];
    range.for_each(|i| {
        let mut file = archive.by_index(i).unwrap();
        let filename = String::from(file.name());
        if file.is_file() &&
            (filename.ends_with(".clj") || 
             filename.ends_with(".cljc") ||
             filename.ends_with(".cljs")) {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            f_and_c.push((filename, contents));
        }
    });
    f_and_c.into_par_iter().for_each(|(filename, contents)| {
        file_count.fetch_add(1, Ordering::Relaxed);
        if !parse_from_bytes(contents.as_bytes()) {
            // XXX: no known satisfactory solution for the following
            println!("{}\t{}", path.to_string_lossy(), filename);
        }
    });
}

fn parse_from_dir(path: &Path, file_count: &AtomicUsize) {
    let mut str = path.to_str().unwrap().to_owned();
    str.push_str("/**/*.*");
    let paths: Vec<Result<PathBuf, _>> = glob(&str).unwrap().collect();
    paths.into_par_iter().for_each(|a_path| {
        let the_path = a_path.unwrap();
        if !the_path.is_dir() {
            parse_dispatch(the_path.as_path(), file_count);
        }
    });
}

fn parse_dispatch(path: &Path, file_count: &AtomicUsize) {
    if path.is_dir() {
        parse_from_dir(path, file_count);
        return;
    }
    if path.is_file() {
        match path.extension() {
            None => return,
            Some(ext) => {
                if ext == "jar" {
                    parse_from_zipfile_path(path, file_count);
                    return;
                }
                if (ext == "clj") || (ext == "cljc") || (ext == "cljs") {
                    parse_from_file_path(path, file_count);
                    return;
                }                
            }
        }
    }
}

fn main() {
    let start = SystemTime::now();
    let args: Vec<String> = env::args().skip(1).collect();
    let file_count = &AtomicUsize::new(0);
    let paths: Vec<&Path> = args.iter().map(|arg| {
        Path::new(arg)
    }).collect();
    paths.into_par_iter().for_each(|path| {
        parse_dispatch(&path, &file_count);
    });
    let since_start = SystemTime::now().duration_since(start)
        .expect("Time went backwards");
    eprintln!("Processed {} files in {}ms. 😎"
              , file_count.load(Ordering::SeqCst)
              , since_start.as_millis());
}
