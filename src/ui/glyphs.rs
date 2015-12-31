use std::collections::HashMap;

use na::{Mat4,Vec2,Vec3,Iso3,Vec4,
         zero, Identity, ToHomogeneous};

use glium::{self,Surface,Display};
use glium::vertex::VertexBufferAny;

use font_atlas::{CharInfo};
use glium::texture::{Texture2d,Texture2dArray,
                     Texture2dDataSource,
                     RawImage2d};

use ::image::GenericImage;

use ui::color;

use ::Font;
use ::ui::atlas::{Atlas};

static VERT_SRC: &'static str = r"
    #version 140

    in vec2 pos;
    in vec2 tex;

    in int visible;

    uniform mat4 transform;

    in vec2 g_pos;
    in vec2 size;
    in vec4 frame;

    out vec2 v_tex_coord;
    out vec2 frame_size;
    out vec2 frame_off;

    out vec2 v_pos;
    in vec4 o_color;
    out vec4 v_color;

    void main() {
        if (visible == 1) { v_pos = pos * size; }
        else { v_pos = vec2(-3000.0,-3000.0); }

        gl_Position = transform * vec4(v_pos + g_pos, 0.0, 1.0);
        v_tex_coord = tex;
        frame_size = frame.zw;
        frame_off = frame.xy;
        v_color = o_color;
    }
";

static FRAG_SRC: &'static str = r"
    #version 140

    uniform sampler2D sample;
    in vec2 v_tex_coord;
    in vec2 frame_size;
    in vec2 frame_off;
   
    in vec4 v_color;
    out vec4 f_color;

    void main() {
        f_color = v_color * texture2D(sample, fract(v_tex_coord) * frame_size + frame_off);
        //f_color = v_color * texture2D(sample, v_tex_coord);
    }
";

#[derive(Copy,Clone)]
pub struct Vertex {
    pub pos: [f32; 2],
    pub tex: [f32; 2],
}

#[derive(Copy, Clone)]
pub struct Attr {
    pub visible: i32,
    pub g_pos: (f32,f32),
    pub size: (f32,f32),
    pub o_color: (f32,f32,f32,f32),
    pub frame: (f32,f32,f32,f32),
}

pub type GlyphCache = HashMap<char,CharInfo>;

pub struct GlyphDrawer {
    vbo: glium::vertex::VertexBufferAny,
    program: glium::Program,
    font: Font,
    //cache:  GlyphCache,
    pub inst: glium::vertex::VertexBuffer<Attr>,
    //index_buf: glium::index::IndexBuffer<u16>,
    sample: Texture2d,
    pub texts: Vec<Text>,
}

// NOTE: Consider using instancing, at least for sentences,
// maybe for all text ever on the screen at once

// FIXME: make like mapdrawer with index buffer and proper coordinates!
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

        let dims = font.image().dimensions();
        let img = font.image_mut().crop(0,0,dims.0,dims.1).flipv();
        
        let sample = Texture2d::new(display,
                                    img).unwrap();
        
        let inst = {
            implement_vertex!(Attr,
                              visible,
                              g_pos,
                              size,
                              o_color,
                              frame);

            let data = vec![
                Attr {
                    visible: 0,
                    g_pos: (0.,0.),
                    size: (0.,0.),
                    o_color: (0.,0.,0.,0.),
                    frame: (0.,0.,0.,0.),
                }
                ;2500];

            glium::vertex::VertexBuffer::dynamic(display, &data).expect("unable to build glyph drawer attr inst vbo")
        };
        
        GlyphDrawer {
            vbo: vbo,
            program: program,
            inst: inst,
            font: font,
            sample: sample,
            texts: vec!(),
        }
    }

    pub fn push(&mut self,text: Text) {
        self.texts.push(text);
    }

    pub fn draw(&mut self,
                pv: Mat4<f32>, // persp*view mat
                target: &mut glium::Frame,) {
        let mut texts = self.texts.drain(..);
        let mut i = 0;

        let mut text = texts.next();
        let mut chars = get_chars(&text);
        
        for q in self.inst.map().iter_mut() {
            
            let c = {
                if i < chars.len() {
                    Some(chars[i])
                }
                else {
                    chars = {
                        text = texts.next();
                        i = 0;
                        get_chars(&text)
                    };

                    if i < chars.len() {
                       Some(chars[i])
                    }
                    else { None }
                }
            };
            i += 1;
            
            let mut img_size: Vec2<f32> = zero();
            q.visible = 0;

            if let Some(c) = c {
                if let Some(cache) = self.font.char_info(c) {
                    if let Some(ref t) = text { // this is technically unwrappable
                        let mut offset_x = 0;
                        if t.center {
                            offset_x = (chars.len() as i32 *
                                        cache.advance.0) / 2;
                        }
                        let pos = Vec2::new((i as f32 *
                                             cache.advance.0 as f32)
                                            - offset_x as f32,
                                            cache.advance.1 as f32) * t.size;

                        img_size = Vec2::new(cache.image_size.0 as f32,
                                             cache.image_size.1 as f32);
                        let img_pos = Vec2::new(cache.image_position.0 as f32,
                                                cache.image_position.1 as f32);
                        
                        let pos = Vec2::new(t.pos.x,t.pos.y) + pos +
                            (img_size * 0.5);

                        /* let translation = Iso3::new(
                        Vec3::new(position.x, position.y, 0.0),
                        Vec3::new(0.0, 0.0, 0.0),
                        );
                        let transform = transform *
                        translation.to_homogeneous();*/
                        
                        q.visible = 1;
                        q.g_pos = (pos.x,pos.y);
                        let size = t.size * img_size;
                        q.size = (size.x,size.y);
                        q.frame = (img_pos.x/256., //(img_pos.x + 0.5)/256.,
                                   img_pos.y/256., //(256. - (img_pos.y + 0.5)) - img_size.y/256.,
                                   img_size.x/256.,
                                   img_size.y/256.);
                        q.o_color = (t.color[0],
                                     t.color[1],
                                     t.color[2],
                                     1.0);
                    }
                }
            }

        }
        
        let params = glium::DrawParameters {
            blend: glium::Blend::alpha_blending(),
            .. Default::default()
        };

        let uniforms = uniform! {
            transform: *pv.as_ref(),
            sample: &self.sample,
        };
        
        target.draw((&self.vbo,self.inst.per_instance().
                     expect("unable to instance glyph drawer")),
                    &glium::index::NoIndices(
                        glium::index::PrimitiveType::TriangleStrip),
                    &self.program, &uniforms, &params).
            expect("glyph drawer target draw failure");

    }

    /*fn load_glyphs (mut font: &mut Font,
                    display: &Display) -> Texture2dArray {
        let mut cache = vec!();

        for c in ascii().into_iter().rev() {
            let g = Atlas::sample_tex(c,
                                      &mut font,
                                      display);
            if let Some(g) = g {
                cache.push(g);
            }

        }

        Texture2dArray::new(display,cache).unwrap()
    }*/

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

fn get_chars (text:&Option<Text>) -> Vec<char> {
    if let &Some(ref t) = text {
        (&*t.text).chars().collect()
    }
    else {
        vec!()
    }
}

#[derive(Clone)]
pub struct Text {
    pub text: String,
    pub size: Vec2<f32>,
    pub color: color::Color,
    pub center: bool,
    pub pos: Vec3<f32>,
}
