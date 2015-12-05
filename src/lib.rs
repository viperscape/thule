extern crate rustc_serialize;
extern crate rand;

#[macro_use] extern crate glium;
extern crate nalgebra as na;

extern crate font_atlas;
extern crate font_atlas_image;
extern crate image;

extern crate clock_ticks;

mod ui;
mod input;
mod events;
mod interface;

pub use font_atlas::{RenderedFont};
pub use image::DynamicImage;
pub type Font = RenderedFont<DynamicImage>;

pub use events::Events;
pub use interface::Interface;
