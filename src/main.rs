use image::{imageops, GenericImageView};
use rayon::prelude::*;
use webp::Encoder;

fn process_image(input: &str, output: &str, quality: f32, scale_factor: f32) {
	let img_result = image::open(input);
    if let Err(e) = img_result {
        println!("Error: {}", e);
        return;
    }
    let img = img_result.unwrap();
    let (w, h) = img.dimensions();
    let img = image::DynamicImage::ImageRgba8(imageops::resize(
        &img,
        (w as f32 * scale_factor) as u32,
        (h as f32 * scale_factor) as u32,
        imageops::FilterType::Triangle,
    ));
    let encoder = Encoder::from_image(&img).unwrap();
    let webp = encoder.encode(quality);
    let output_path = std::path::Path::new(output);
    std::fs::write(output_path, &*webp).expect("Unable to write file");
}

fn process_directory(input_dir: &str, output_dir: &str, quality: f32, scale_factor: f32) {
	let entries: Vec<_> = std::fs::read_dir(input_dir).expect("Unable to read directory").collect();
	entries.par_iter().for_each(|entry| {
		if let Ok(entry) = entry {
			let path = entry.path();
			if path.is_file() {
				if let Some(extension) = path.extension() {
					if extension == "png" || extension == "jpg" || extension == "jpeg" {
						let input = path.to_str().unwrap();
						let output = format!("{}/{}.webp", output_dir, path.file_stem().unwrap().to_str().unwrap());
						process_image(input, &output, quality, scale_factor);
					}
				}
			}
		}
	});
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <input> <output> [--batch] [--quality=VALUE]", args[0]);
        return;
    }

    let is_batch = args.contains(&"--batch".to_string());

    let mut quality = 100.0;
    let mut scale_factor = 1.0;

    let quality_arg = args.iter().find(|&arg| arg.starts_with("--quality="));
    if let Some(q_arg) = quality_arg {
        let parts: Vec<&str> = q_arg.split('=').collect();
        if parts.len() == 2 {
            quality = parts[1].parse::<f32>().expect("Invalid quality value");
        }
    }

    if !args.iter().any(|arg| arg.starts_with("--quality=")) && args.len() > 4 {
        scale_factor = args[4]
            .parse::<f32>()
            .expect("Invalid scale factor (0.0-100.0)");
    }

    if is_batch {
        let input_dir = &args[1];
        let output_dir = &args[2];
        process_directory(input_dir, output_dir, quality, scale_factor);
    } else {
        let input = &args[1];
        let output = &args[2];
        process_image(input, output, quality, scale_factor);
    }
}