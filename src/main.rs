use std::fs::File;
use std::path::Path;
use std::process;

extern crate clap;
use clap::{App, AppSettings, Arg, SubCommand};

extern crate serde;
use serde::ser::Serialize;

extern crate serde_json;
use serde_json::Value;

fn main() {
    let matches = App::new("JPatch")
        .version("0.1")
        .about("Patches JSON file")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(
            Arg::with_name("TARGET")
                .help("File to patch")
                .required(true)
                .index(1),
        )
        .subcommand(
            SubCommand::with_name("merge")
                .about("merges another file")
                .arg(Arg::with_name("SOURCE").required(true).index(1)),
        )
        .get_matches();

    let target_fn = matches.value_of("TARGET").unwrap();

    if !Path::new(target_fn).exists() {
        println!("Cannot find {}", target_fn);
        process::exit(2);
    }

    let merge_matches = matches.subcommand_matches("merge").unwrap();
    let source_fn = merge_matches.value_of("SOURCE").unwrap();

    if !Path::new(source_fn).exists() {
        println!("Cannot find {}", source_fn);
        process::exit(2);
    }

    println!("Merging {} into {}", source_fn, target_fn);

    let target_file = File::open(target_fn).unwrap();
    let source_file = File::open(source_fn).unwrap();

    let mut target: Value = serde_json::from_reader(target_file).unwrap();
    let source: Value = serde_json::from_reader(source_file).unwrap();

    merge(&mut target, &source);

    let result_file = File::create(target_fn).unwrap();

    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
    let mut ser = serde_json::Serializer::with_formatter(result_file, formatter);

    target.serialize(&mut ser).unwrap();
}

fn merge(target: &mut Value, source: &Value) {
    match (target, source) {
        (&mut Value::Object(ref mut a), &Value::Object(ref b)) => {
            for (k, v) in b {
                merge(a.entry(k.clone()).or_insert(Value::Null), v);
            }
        }
        (a, b) => {
            *a = b.clone();
        }
    }
}
