use crossbeam_utils::sync::WaitGroup;
use glob::glob;
use lexical_sort::{natural_lexical_cmp, StringSort};
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::thread;
use time::PreciseTime;
use yaml_rust::YamlLoader;

fn main() -> std::io::Result<()> {
    let start = PreciseTime::now();
    let with_sorting = false;
    let outdir = "words";
    fs::create_dir_all(outdir)?;
    let wg = WaitGroup::new();
    let path = "../data/??/**/*.yml";
    // let path = "./bibles/??/**/*.yml";
    for entry in glob(path).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                // let separator = Regex::new(r"[^\p{L}]+").unwrap();
                let separator = Regex::new(r"[\W\d]+").unwrap();
                let wg = wg.clone();
                thread::spawn(move || {
                    let filepath = path.to_str().unwrap().replace(".yml", ".txt");
                    println!("{:?}", filepath);
                    let text = fs::read_to_string(&filepath)
                        .unwrap()
                        .to_lowercase()
                        .replace("\n", " ");
                    let tokens: Vec<&str> = separator.split(&text).collect();
                    let unique_tokens: HashSet<&str> = tokens.into_iter().collect();
                    let mut words: Vec<&str>;
                    if with_sorting {
                        words = unique_tokens.into_iter().collect();
                        words.string_sort_unstable(natural_lexical_cmp);
                    } else {
                        words = unique_tokens.into_iter().collect();
                    }
                    let yaml = fs::read_to_string(&path).unwrap();
                    let docs = YamlLoader::load_from_str(&yaml).unwrap();
                    let meta = &docs[0];
                    let out = format!(
                        "{}/extracted-words-for-{}.txt",
                        outdir,
                        meta["label"].as_str().unwrap()
                    );
                    fs::write(out, words.join("\n"));

                    drop(wg);
                });
            }
            Err(e) => println!("{:?}", e),
        }
    }
    wg.wait();
    let end = PreciseTime::now();
    println!("{:?} seconds.", start.to(end));
    Ok(())
}

//  2s