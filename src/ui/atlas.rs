use std::fs::File;

use font_atlas::{RenderedFont,CharInfo};
use font_atlas_image::{read_atlas};
use image::{DynamicImage};

use glium::texture::{Texture2d};
use glium::Display;

use ::Font;

pub struct Atlas;

impl Atlas {
    pub fn new (path: &str) -> Option<RenderedFont<DynamicImage>> {
        let atlas_img = format!("{}.png",path);
        let atlas_meta = format!("{}.json",path);
        
         if let Some(mut atlas_img) = File::open(atlas_img).ok() {
             if let Some(mut atlas_meta) = File::open(atlas_meta).ok() {
                 let data =
                     read_atlas(&mut atlas_img,
                                &mut atlas_meta).ok().
                     expect("Error loading atlas!");
                   
                 return Some(data)

             }
         }
        None
    }

    pub fn sample_tex (c: char,
                       font: &mut Font,
                       display: &Display) -> Option<(CharInfo,Texture2d)> {
        if let Some(c) = font.char_info(c) {
            if c.image_size.0 == 0 { return None } //no char found!
            let img = font.image_mut();
            let img = img.crop(c.image_position.0,
                               c.image_position.1,
                               c.image_size.0,
                               c.image_size.1).flipv();
            if let Some(tex) = Texture2d::new(display, img).ok() {
                return Some((c,tex))
            }
        }
        None 
    }
}
