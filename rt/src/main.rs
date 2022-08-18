use bytes::{BufMut, BytesMut};
use chrome_native::{ChromeNativeErrors, Plugin, ERRORCODE_FAIL, ERRORCODE_OK};
use clap::{Parser, Subcommand};
use libloading::{Library, Symbol};
use serde_json::json;
use std::{
    fs::{canonicalize, File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Read, Write},
    path::Path,
};

const STDIO_MAX_BUFFER_SIZE: usize = 1024;

fn load(library: &str) -> (Library, Box<dyn Plugin>) {
    unsafe {
        let lib = libloading::Library::new(library).unwrap();
        let plugin = {
            let get_plugin: Symbol<extern "C" fn() -> *mut dyn Plugin> =
                lib.get(b"get_plugin").unwrap();
            get_plugin()
        };
        (lib, Box::from_raw(plugin))
    }
}
#[derive(Subcommand, Debug)]
enum SubCommand {
    AddLibrary { library_path: String },
    RemoveLibrary { library_path: String },
    ListLibraries,
}

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    #[clap(subcommand)]
    action: Option<SubCommand>,
    #[clap(long, value_parser)]
    parent_window: Option<u32>,
}

fn read_db(file_reader: &mut BufReader<File>, libraries: &mut Vec<String>) {
    let mut line = String::new();

    loop {
        let read_bytes = file_reader.read_line(&mut line).unwrap();
        if read_bytes <= 1 {
            break;
        }
        let path = line.strip_suffix("\n").unwrap();
        let path = Path::new(&path);
        if path.exists() {
            libraries.push(
                canonicalize(path.clone())
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
            );
        }
        line.clear();
    }
}

fn write_db(file: &File, file_writer: &mut BufWriter<&File>, libraries: &Vec<String>) {
    file.set_len(0).unwrap();
    for library in libraries.iter() {
        file_writer
            .write_all(format!("{}\n", library).as_bytes())
            .unwrap();
        file_writer.flush().unwrap();
    }
}

fn main() {
    std::env::set_current_dir(std::env::current_exe().unwrap().parent().unwrap()).unwrap();

    let mut libraries = vec![];

    let read_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("../libraries")
        .unwrap();
    let mut reader = BufReader::new(read_file);

    read_db(&mut reader, &mut libraries);

    let write_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("../libraries")
        .unwrap();
    let mut writer = BufWriter::new(&write_file);

    write_db(&write_file, &mut writer, &libraries);

    let raw_env = std::env::args().any(|a| a.contains("chrome-extension://"));
    if !raw_env {
        let parsed = Args::parse();
        if let Some(action) = parsed.action {
            match action {
                SubCommand::AddLibrary { library_path } => {
                    if let Ok(canon_path) = std::fs::canonicalize(library_path) {
                        if !libraries
                            .iter()
                            .any(|path| std::fs::canonicalize(path).unwrap() == canon_path)
                        {
                            libraries.push(canon_path.to_str().unwrap().to_string());
                            write_db(&write_file, &mut writer, &libraries);
                        }
                    }
                }
                SubCommand::RemoveLibrary { library_path } => {
                    let canon = canonicalize(library_path)
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string();
                    libraries = libraries
                        .iter()
                        .filter(|&path| path.as_str() != canon.as_str())
                        .cloned()
                        .collect::<Vec<String>>();
                    write_db(&write_file, &mut writer, &libraries);
                }
                SubCommand::ListLibraries => {
                    println!("Libraries\n");
                    for library in libraries.iter() {
                        println!("- {}", library);
                    }
                }
            }
            return;
        }
    }
    // "C:/Users/dolph/Desktop/chrome-native/target/release/example.dll"
    let mut plugins = vec![];
    for library in libraries.iter() {
        plugins.push(load(library));
    }

    let mut stdin = BufReader::new(std::io::stdin().lock());
    let mut stdout = BufWriter::new(std::io::stdout().lock());

    let mut read_buf = BytesMut::with_capacity(STDIO_MAX_BUFFER_SIZE);
    let mut write_buf = BytesMut::with_capacity(STDIO_MAX_BUFFER_SIZE);
    read_buf.resize(STDIO_MAX_BUFFER_SIZE, 0);

    let read_bytes = stdin.read(&mut read_buf[..]).unwrap();
    let mut iter = plugins.iter();
    let mut result: Result<String, Box<dyn std::error::Error>> =
        Err("Couldn't find the right handler".into());
    while let Some((_, plugin)) = iter.next() {
        result =
            plugin.handle_command(String::from_utf8(read_buf[4..read_bytes].to_vec()).unwrap());
        if let Err(ref e) = result {
            if !e.is::<ChromeNativeErrors>() {
                break;
            }
        }
    }
    let (code, message) = if result.is_ok() {
        (ERRORCODE_OK, result.unwrap())
    } else {
        (ERRORCODE_FAIL, result.err().unwrap().to_string())
    };

    let result = json!({
        "result": {
            "code": code,
            "message": message
        }
    });
    let response_str = result.to_string();
    let response_bytes = response_str.as_bytes();

    write_buf.put_u32_le(response_bytes.len() as u32);
    write_buf.put(response_bytes);

    stdout.write_all(&write_buf[..]).unwrap();
    stdout.flush().unwrap();

    // Dereference plugin and library in order
    for (lib, plugin) in plugins {
        {
            plugin
        };
        {
            lib
        };
    }

    write_buf.clear();
    read_buf.clear();
}
