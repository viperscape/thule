#![allow(dead_code)]
#![feature(drain)]

extern crate rustc_serialize;
extern crate rand;
extern crate bincode;

#[macro_use] extern crate glium;
extern crate nalgebra as na;
extern crate ncollide as nc;

extern crate font_atlas;
extern crate font_atlas_image;
extern crate image;

extern crate clock_ticks;

extern crate obj;
extern crate genmesh;

extern crate hex2d;

extern crate noise;
extern crate toml;

mod ui;
mod input;
mod events;
mod interface;
mod grid;
mod game;
mod config;

pub use font_atlas::{RenderedFont};
pub use image::DynamicImage;
pub type Font = RenderedFont<DynamicImage>;

pub use events::Events;
pub use interface::Interface;
pub use grid::{Grid,GridGroup,TileKind,Tile,
               GRIDSIZE,GROUPSIZE,MAPSIZE};
pub use game::{GameState,Player};
pub use input::Keyboard;
pub use config::{Bindings,Default};
pub use ui::Camera;
