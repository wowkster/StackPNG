use image::ImageFormat;
use question::{Answer, Question};

use stackpng::{args::parse_args, common::print_error, create_mc_meta, process_images};

fn main() {
    let Ok(args) = parse_args() else {
        print_error("Could not parse arguments!")
    };

    // Stitch the images together
    let image_res = process_images(&args);

    // Make sure the image path is valid
    let img_path = get_path(&args.name, "png");

    // Try saving the file
    if let Err(e) = image_res.image.save_with_format(img_path, ImageFormat::Png) {
        print_error(format!("Error saving image file: {e:?}"))
    };

    // Create MCMeta file
    if !args.disable_mcmeta {
        // Make sure the mcmeta path is valid
        let mcmeta_path = get_path(&args.name, "mcmeta");

        // Create the resulting meta file
        let mcmeta = create_mc_meta(&args, &image_res);

        // Try saving the mcmeta file
        if let Err(e) = std::fs::write(mcmeta_path, mcmeta) {
            print_error(format!("Error saving mcmeta file: {e:?}"))
        };
    }
}

fn get_path<S>(name: S, extension: &str) -> std::path::PathBuf
where
    S: ToString + Sized,
{
    let path = std::path::PathBuf::from(format!("./{}.{}", name.to_string(), extension));

    if path.is_dir() {
        print_error("Path {path:?} already exists and is a directory. Specify a different output name with the `--name` argument")
    }

    if path.exists() {
        let res = Question::new(
            format!(
                "File {:?} already exists. Do you want to overwrite the file?",
                path
            )
            .as_str(),
        )
        .confirm();

        if matches!(res, Answer::NO) {
            println!("Exiting...");
            std::process::exit(0);
        }
    }

    path
}
