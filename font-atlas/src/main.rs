extern crate font_atlas;
extern crate font_atlas_freetype as freetype_atlas;
extern crate font_atlas_image as image_atlas;
extern crate freetype;
extern crate image;

use std::vec;

fn ascii() ->  vec::IntoIter<char> {
    let mut v = Vec::with_capacity(256);
    for i in 0u8 .. 255 {
        let c = i as char;
        if !c.is_control() {
            v.push(c);
        }
    }
    v.into_iter()
}

// ./atlas_generator SomeFont.ttf 38 20 10 OtherFont.ttf 20 10 30
fn main() {
    let mut targets: Vec<(String, Vec<u32>)> = vec![];

    for arg in std::env::args().skip(1) {
        match arg.parse::<u32>().ok() {
            Some(i) => {
                if let Some(last) = targets.last_mut() {
                    last.1.push(i);
                } else {
                    println!("ERROR: The size {} must be matched to a font", i);
                    std::process::exit(1);
                }
            }
            None => {
                targets.push((arg, vec![]));
            }
        }
    }

    for target in &targets {
        if target.1.len() == 0 {
            println!("ERROR: The font {} has no given sizes!", target.0);
            std::process::exit(2);
        }
    }

    let library = freetype::Library::init().ok().expect("Freetype library failed to open!");

    for (file, sizes) in targets {
        render_face(&library, file, sizes);
    }
}

fn render_face(library: &freetype::Library, file: String, sizes: Vec<u32>)  {
    if let Some(mut face) = library.new_face(file.clone(), 0).ok() {
        for size in sizes {
            let _ = face.set_pixel_sizes(0, size);
            match freetype_atlas::render(&mut face, ascii(), true) {
                Ok(rendered) => {
                    let name = file.split('.').nth(0).unwrap_or(&file[..]);
                    let img_path = format!("{}-{}.png", name, size);
                    let meta_path = format!("{}-{}.json", name, size);
                    image_atlas::save_atlas(rendered, image::ImageFormat::PNG, img_path, meta_path).ok().expect("Unable to save atlas");
                }
                Err(_) => panic!("Atlas unable to render"),
            }
        }
    }
    else { panic!("No font face found") }
}
