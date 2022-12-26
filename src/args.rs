use std::collections::VecDeque;

use crate::common::print_error;

#[derive(Debug)]
pub struct StackPngArgs {
    pub name: String,
    pub frame_time: u16,
    pub disable_mcmeta: bool,
    pub files: Vec<String>,
}

pub fn parse_args() -> Result<StackPngArgs, ()> {
    let mut args: VecDeque<_> = std::env::args().collect();

    // Remove the executable name arg
    args.pop_front().unwrap();

    if args.len() == 0 {
        print_help();
    }

    let mut name = None;
    let mut frame_time = None;
    let mut disable_mcmeta = None;
    let mut ignore_invalid = false;
    let mut files = None;

    while !args.is_empty() {
        match args.front().unwrap().as_str() {
            "-h" | "--help" => {
                print_help();
            }
            "-n" | "--name" => {
                args.pop_front().unwrap();

                let Some(output) = args.pop_front() else {
                    print_error("Expected name after name flag!");
                };

                name = Some(output);
            }
            "-f" | "--frame-time" => {
                args.pop_front().unwrap();

                let Some(ticks) = args.pop_front() else {
                    print_error("Expected ticks after frame time flag!");
                };

                let Ok(ticks) = ticks.parse::<u16>() else {
                    print_error("Argument after frame time flag is not an integer!");
                };

                frame_time = Some(ticks);
            }
            "-d" | "--disable-mcmeta" => {
                args.pop_front().unwrap();

                disable_mcmeta = Some(true);
            }
            "-i" | "--ignore-invalid" => {
                args.pop_front().unwrap();

                ignore_invalid = true;
            }
            _ => break,
        }
    }

    // Check if a directory was passed, and if so read it's contents
    if args.len() == 1 {
        let path_name = args.pop_front().unwrap();

        let Ok(meta) = std::fs::metadata(&path_name) else {
            print_error(format!("Could not read path `{}`!", &path_name));
        };

        if meta.is_dir() {
            // Read the dir
            let Ok(folder_files) = std::fs::read_dir(&path_name) else {
                print_error("Could not read provided directory!");
            };

            let mut res = Vec::new();

            for file in folder_files {
                let Ok(file) = file else {
                    print_error(format!("Could not access file `{}`!", &path_name));
                };

                let Ok(meta) = file.metadata() else {
                    print_error(format!("Could not access `{}`!", &path_name));
                };

                if meta.is_dir() {
                    continue;
                }

                let file_name = file.path().to_str().unwrap().to_owned();

                if is_valid_png_path(&file_name) {
                    res.push(file_name);
                } else if !ignore_invalid {
                    print_error(format!("File `{}` is not a valid PNG file!", file_name));
                }
            }

            files = Some(res);
        } else {
            if is_valid_png_path(&path_name) {
                // Add as only file to files
                files = Some(vec![path_name])
            } else if !ignore_invalid {
                print_error(format!("File `{}` is not a valid PNG file!", path_name));
            }
        }
    } else {
        let mut res = Vec::new();

        while !args.is_empty() {
            let file = args.pop_front().unwrap();

            let Ok(meta) = std::fs::metadata(&file) else {
                print_error(format!("Could not access `{}`!", &file));
            };

            if !meta.is_file() {
                print_error(format!(
                    "Can not mix and match files and folders. `{}` is a directory!",
                    &file
                ));
            }

            if is_valid_png_path(&file) {
                res.push(file);
            } else if !ignore_invalid {
                print_error(format!("File `{}` is not a valid PNG file!", file));
            }
        }

        files = Some(res);
    }

    if files.is_none() || files.as_ref().unwrap().len() == 0 {
        print_error("No valid PNG files provided!");
    }

    Ok(StackPngArgs {
        name: name.unwrap_or(String::from("animation")),
        frame_time: frame_time.unwrap_or(2),
        disable_mcmeta: disable_mcmeta.unwrap_or(false),
        files: files.unwrap(),
    })
}

pub fn is_valid_png_path(path: &String) -> bool {
    let Ok(meta) = std::fs::metadata(path) else {
        return false;
    };

    if !meta.is_file() {
        return false;
    }

    if !path.ends_with(".png") {
        return false;
    }

    true
}

pub fn print_help() -> ! {
    println!("StackPNG v1.0.0");
    println!();
    println!("USAGE:");
    println!("    stackpng [...options] <folder>                        - Constructs a stacked image from all the files in a folder (alphabetically)");
    println!("    stackpng [...options] <...files>                      - Constructs a stacked image from all the provided PNG files (in order given)");
    println!("    stackpng --frame-time 4 --ignore-invalid ./anim");
    println!();
    println!("OPTIONS:");
    println!("    -h, --help                    - Show the help menu");
    println!("    -n, --name <name>             - Specify the output file name");
    println!("    -f, --frame-time <ticks>      - Specify the animation frame length");
    println!("    -i, --ignore-invalid          - Ignore any non PNG files in the input");
    println!("    -d, --disable-mcmeta          - Do not emit MCMeta file");

    std::process::exit(0);
}
