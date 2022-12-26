mod args;
mod common;

use args::{parse_args, StackPngArgs};
use common::print_error;

fn main() {
    let Ok(args) = parse_args() else {
        print_error("Could not parse arguments!")
    };

    println!("{:#?}", args);

    process_images(args);
}



fn process_images(args: StackPngArgs) {
    let img_results: Vec<_> = args.files.iter().map(|f| {
        match image::open(&f) {
            Err(e) => print_error(format!("Could not open/parse image `{}`!", f)),
            Ok(i) => i
        }
    }).collect();

    if !img_results.iter().
}



