use std::{env, fs, io, path::PathBuf};

use image::{DynamicImage, RgbaImage};
use log::{error, warn};

fn find_images() -> io::Result<Vec<PathBuf>> {
    let dir = PathBuf::from(env::args().last().unwrap());
    let mut ret = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if let Some(ext) = path.extension() {
            if ext == "png" {
                ret.push(path.clone());
            }
        }
    }

    ret.sort();
    Ok(ret)
}

fn open_images(image_path_vec: Vec<PathBuf>) -> Vec<RgbaImage> {
    image_path_vec
        .into_iter()
        .map(|p| (p.clone(), image::open(p).unwrap()))
        .map(|(p, i)| match i {
            DynamicImage::ImageRgba8(img) => Some(img),
            _ => {
                error!("{:?} is not a valid image (RBGA8)", p);
                None
            }
        })
        .flatten()
        .collect()
}

fn pack_images(image_vec: Vec<RgbaImage>) -> Option<RgbaImage> {
    if image_vec.len() <= 1 {
        error!("at least two images are expected");
        return None;
    }

    let dim = image_vec[0].dimensions();
    for img in &image_vec {
        if img.dimensions() != dim {
            error!(
                "image dimension {:?} is not equal to expected {:?}",
                img.dimensions(),
                dim
            );
            return None;
        }
    }

    let row_cnt = f32::sqrt(image_vec.len() as f32).ceil() as u32;
    let packed_img = RgbaImage::from_fn(row_cnt * dim.0, row_cnt * dim.1, |x, y| {
        let x_idx = x / dim.0;
        let x_offset = x % dim.0;
        let y_idx = y / dim.1;
        let y_offset = y % dim.1;

        if let Some(cell_img) = image_vec.get((x_idx + y_idx * row_cnt) as usize) {
            cell_img.get_pixel(x_offset, y_offset).clone()
        } else {
            image::Rgba([0, 0, 0, 0])
        }
    });

    Some(packed_img)
}

fn main() {
    let image_path_vec = find_images().unwrap();
    println!("find images:\n {:#?}", image_path_vec);

    let image_vec = open_images(image_path_vec);
    if let Some(packed_image) = pack_images(image_vec) {
        packed_image.save("result.png").unwrap();
    } else {
        warn!("no images are packed")
    }
}
