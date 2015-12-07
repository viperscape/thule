use std::collections::HashMap;

use na::{Mat4,Vec2,Vec3,Iso3,Vec4, ToHomogeneous};

use glium::{self,Surface,Display};
use glium::vertex::VertexBufferAny;

use font_atlas::{CharInfo};
use glium::texture::{Texture2d};

use ui::color;

use ::Font;
use ui::atlas::{Atlas};

static VERT_SRC: &'static str = r"
    #version 140

    in vec2 pos;
    in vec2 tex;

    uniform mat4 transform;
    uniform vec2 size;

    out vec2 v_tex_coord;

    void main() {
        gl_Position = transform * vec4(pos * size, 0.0, 1.0);
        v_tex_coord = tex;
    }
";

static FRAG_SRC: &'static str = r"
    #version 140

    in vec2 v_tex_coord;

    uniform sampler2D sample;
    uniform vec4 o_color;

    void main() {
        gl_FragColor = o_color * texture2D(sample, v_tex_coord);
    }
";

#[derive(Copy,Clone)]
pub struct Vertex {
    pub pos: [f32; 2],
    pub tex: [f32; 2],
}

pub type GlyphCache = HashMap<char,(CharInfo,Texture2d)>;

pub struct GlyphDrawer {
    vbo: glium::vertex::VertexBufferAny,
    program: glium::Program,
    font: Font,
    cache:  GlyphCache,
}

impl GlyphDrawer {
    pub fn new(mut font: Font, display: &Display) -> GlyphDrawer {
        implement_vertex!(Vertex, pos, tex);
        let verts = vec![
            Vertex { pos: [ -0.5,  0.5 ], tex: [ 0.0, 0.0 ] },
            Vertex { pos: [ -0.5, -0.5 ], tex: [ 0.0, 1.0 ] },
            Vertex { pos: [  0.5,  0.5 ], tex: [ 1.0, 0.0 ] },
            Vertex { pos: [  0.5, -0.5 ], tex: [ 1.0, 1.0 ] },
            ];
        
        let program = program!(display,
                               140 => { vertex: VERT_SRC,
                                        fragment: FRAG_SRC, } ).unwrap();
        let vbo = glium::vertex::VertexBuffer::new(display, &verts).unwrap().into_vertex_buffer_any();

        let cache = GlyphDrawer::load_glyphs(&mut font, display);
        
        GlyphDrawer {
            vbo: vbo,
            program: program,
            font: font,
            cache: cache,
        }
    }
    
    pub fn draw(
        &mut self,
        text: &str,
        size: Vec2<f32>,
        color: color::Color,
        center: bool,
        transform: Mat4<f32>,
        target: &mut glium::Frame,
        ) {
        let width = text.chars().count();

        for (i, c) in text.chars().enumerate() {
            if let Some(cache) = self.cache.get(&c) {
                let mut offset_x = 0;
                if center {
                    offset_x = (width as i32 * cache.0.advance.0) / 2;
                }
                let position = Vec2::new((i as f32 *
                                          cache.0.advance.0 as f32)
                                         - offset_x as f32,
                                         0.0) * size;

                let img_size = Vec2::new(cache.0.image_size.0 as f32,
                                         cache.0.image_size.1 as f32);
                
                let position = position +
                    (img_size * 0.5);

                let translation = Iso3::new(
                    Vec3::new(position.x, position.y, 0.0),
                    Vec3::new(0.0, 0.0, 0.0),
                    );
                let transform = transform * translation.to_homogeneous();
                
                let uniforms = uniform! {
                    transform: *transform.as_array(),
                    size: *(size * img_size).as_array(),
                    sample: &cache.1,
                    o_color: *Vec4::new(color[0],color[1],color[2],1.0).as_array(),
                };

                // draw parameters
                let params = glium::DrawParameters {
                    blend: glium::Blend::alpha_blending(),
                    .. Default::default()
                };

                target.draw(&self.vbo,
                            &glium::index::NoIndices
                            (glium::index::PrimitiveType::TriangleStrip),
                            &self.program, &uniforms, &params).unwrap();
            }
            else if c!=' ' { println!("{:?}, no char found",c); }
        }
    }

    fn load_glyphs (font: &mut Font,
                    display: &Display) -> GlyphCache {
        let mut cache = HashMap::new();

        for c in ascii().into_iter() {
            let g = Atlas::sample_tex(c,
                                      font,
                                      display);
            if let Some(g) = g {
                cache.insert(c,g);
            }

        }
        
        cache
    }

    pub fn new_from_path(path: &str, display: &Display) -> GlyphDrawer {
        let atlas = Atlas::new(path).expect("Font atlas cannot load, missing fonts?");

        GlyphDrawer::new(atlas,display)
    }
}


fn ascii() ->  Vec<char> {
    let mut v = Vec::with_capacity(256);
    for i in 0u8 .. 255 {
        let c = i as char;
        if !c.is_control() {
            v.push(c);
        }
    }
    v
}
