pub mod args;
pub mod common;

use crate::args::StackPngArgs;
use crate::common::print_error;

use image::{imageops::FilterType, DynamicImage, ImageBuffer, Rgba};
use serde_json::json;

pub struct ImageResult {
    pub image: DynamicImage,
    pub width: u32,
    pub height: u32,
}

pub fn process_images(args: &StackPngArgs) -> ImageResult {
    let mut img_results: Vec<_> = args
        .files
        .iter()
        .map(|f| match image::open(&f) {
            Err(e) => print_error(format!("{e:?}\n\nCould not open/parse image `{}`!", f)),
            Ok(i) => i,
        })
        .collect();

    let first = img_results.first().unwrap();

    let width = first.width();
    let height = first.height();

    // Check if all the images have the same dimensions
    if !img_results
        .iter()
        .all(|i| i.width() == width && i.height() == height)
    {
        // If resizing is not allowed, throw an error
        if !args.resize {
            print_error(format!(
                "All images in sequence must have the same aspect ratio!\n\
                To allow resizing images with different dimensions, use the `--resize` flag.\n\
                \n\
                First image in sequence has dimensions {width}x{height} (WxH), but not all images matched these dimensions!",
            ))
        }

        // Resize all images to match the aspect ratio of the first image
        img_results.iter_mut().for_each(|i| {
            // If the image already has the correct dimensions, do not resize it
            if i.width() == width && i.height() == height {
                return;
            }

            /*
             If the `--ignore-aspect-ratio` argument is provided,
             otherwise resize to the exact dims, otherwise preserve the aspect ratio
            */
            *i = if args.ignore_aspect_ratio {
                i.resize_exact(width, height, FilterType::Nearest)
            } else {
                i.resize(width, height, FilterType::Nearest)
            }
        })
    }

    if !img_results
        .iter()
        .all(|i| i.width() == width && i.height() == height)
    {
        print_error(format!(
            "All images in sequence must have the same dimensions!\n\
            Tried to resize preserving aspect ratios, but failed since not all images had the same aspect ratio. Use the `--ignore-aspect-ratio` flag to force images to resize.\n\
            \n\
            First image in sequence has dimensions {width}x{height} (WxH) and aspect ratio {:.3?}, but not all images matched these dimensions!",
            width as f64 / height as f64
        ))
    }

    let mut img = ImageBuffer::from_fn(width, height * img_results.len() as u32, |_, _| {
        Rgba([0, 0, 0, 0])
    });

    for (index, image) in img_results.iter().enumerate() {
        image::imageops::overlay(&mut img, image, 0, (height * (index as u32)) as i64);
    }

    ImageResult {
        image: DynamicImage::from(img),
        width,
        height,
    }
}

pub fn create_mc_meta(args: &StackPngArgs, image: &ImageResult) -> String {
    let mcmeta = json!({
         "animation": {
            "frametime": args.frame_time,
            "width": image.width,
            "height": image.height
         }
    });

    mcmeta.to_string()
}
