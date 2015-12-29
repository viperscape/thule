use na::{Mat4,Vec2,Vec3,Iso3,Vec4,
         zero, Identity, ToHomogeneous};

use glium::{self,Surface,Display};
use glium::vertex::VertexBufferAny;

use font_atlas::{CharInfo};
use glium::texture::{Texture2d,Texture2dArray,Texture2dDataSource,RawImage2d};

use ui::color;

use ::Font;
use ::ui::atlas::{Atlas};

static VERT_SRC: &'static str = r"
    #version 140

    in vec2 pos;
    in vec2 tex;

    in int visible;
    in uint sample_id;

    uniform mat4 transform;

    in vec2 g_pos;
    in vec2 size;

    out vec2 v_tex_coord;
    out vec2 v_pos;
    flat out uint v_tex_id;
    //in vec4 o_color;

    void main() {
v_pos = pos * size;
        if (visible == 1) { v_pos = pos * size; }
        else { v_pos = vec2(-3000.0,-3000.0); }

        gl_Position = transform * vec4(v_pos, 0.0, 1.0);
        v_tex_coord = tex;
        v_tex_id = sample_id;
        //v_color = o_color;
    }
";

static FRAG_SRC: &'static str = r"
    #version 140

    uniform sampler2DArray sample;
    in vec2 v_tex_coord;
    flat in uint v_tex_id;
    
    in vec4 o_color;
    out vec4 f_color;

    void main() {
        texture(sample, vec3(v_tex_coord, float(v_tex_id)));
        f_color = texture(sample, vec3(v_tex_coord, float(v_tex_id)));

        //texture2D(sample, v_tex_coord);
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
    pub sample_id: u32,
}

pub type GlyphCache<'a> = Vec<RawImage2d<'a,u8>>;

pub struct GlyphDrawer {
    vbo: glium::vertex::VertexBufferAny,
    program: glium::Program,
    //_font: Font,
    //cache:  GlyphCache,
    pub inst: glium::vertex::VertexBuffer<Attr>,
    //index_buf: glium::index::IndexBuffer<u16>,
    samples: Texture2dArray,
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

        let cache = GlyphDrawer::load_glyphs(font, display);
        //let glyphs = cache.iter().map(|n| n.1).collect();
        let samples = Texture2dArray::new(display,
                                          cache).unwrap();
        
        let inst = {
            implement_vertex!(Attr,
                              visible,
                              g_pos,
                              size,
                              o_color);

            let data = vec![
                Attr {
                    visible: 0,
                    g_pos: (0.,0.),
                    size: (0.,0.),
                    o_color: (0.,0.,0.,0.),
                    sample_id: 0,
                }
                ;5000];

            glium::vertex::VertexBuffer::dynamic(display, &data).expect("unable to build glyph drawer attr inst vbo")
        };
        
        GlyphDrawer {
            vbo: vbo,
            program: program,
            //_font: font,
            //cache: cache,
            inst: inst,
            samples: samples,
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
            i += 1;
            let c = {
                if i < chars.len() {
                    Some(chars[i-1])
                }
                else {
                    chars = {
                        text = texts.next();
                        i = 0;
                        get_chars(&text)
                    };

                    if i < chars.len() {
                        Some(chars[i-1])
                    }
                    else { None }
                }
            };

            let mut img_size: Vec2<f32> = zero();

            /*
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
                let transform = transform *
                    translation.to_homogeneous();
            }
            else if c!=' ' { println!("{:?}, no char found",c); }*/
            
            if let Some(c) = c {
                if let Some(ref t) = text {
                    q.visible = 1;
                    q.g_pos = (t.pos.x,t.pos.y);
                    let size = t.size * img_size;
                    q.size = (size.x,size.y);
                    q.sample_id = c as u32;
                    q.o_color = (t.color[0],
                                 t.color[1],
                                 t.color[2],
                                 1.0);
                }
                else {
                    q.visible = 0;
                }
            }
            else {
                q.visible = 0;
            }

        }
        
        let params = glium::DrawParameters {
            blend: glium::Blend::alpha_blending(),
            .. Default::default()
        };

        let uniforms = uniform! {
            transform: *pv.as_ref(),
            samples: &self.samples,
        };
        
        target.draw((&self.vbo,self.inst.per_instance().
                     expect("unable to instance glyph drawer")),
                    &glium::index::NoIndices(
                        glium::index::PrimitiveType::TriangleStrip),
                    &self.program, &uniforms, &params).
            expect("glyph drawer target draw failure");

    }

    fn load_glyphs<'a> (mut font: Font,
                        display: &Display) -> GlyphCache<'a> {
        let mut cache = vec!();

        for c in ascii().into_iter().rev() {
            let g = Atlas::sample_tex(c,
                                      &mut font,
                                      display);
            if let Some(g) = g {
                cache.push(g);
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
