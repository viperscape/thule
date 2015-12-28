use std::path::Path;

use na::{Mat4,Vec2,Vec4};

use glium::{self,Surface,Display};
use glium::vertex::VertexBufferAny;

use glium::texture::{Texture2d};

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

    void main() {
        gl_FragColor = texture2D(sample, v_tex_coord);
    }
";

#[derive(Copy,Clone)]
pub struct Vertex {
    pub pos: [f32; 2],
    pub tex: [f32; 2],
}

pub struct MapDrawer {
    vbo: glium::vertex::VertexBufferAny,
    program: glium::Program,
    tex: Texture2d,
}

impl MapDrawer {
    pub fn new(path: &str, display: &Display) -> MapDrawer {
        implement_vertex!(Vertex, pos, tex);
        /*let verts = vec![
            Vertex { pos: [ -0.5,  0.5 ], tex: [ 0.0, 0.0 ] },
            Vertex { pos: [ -0.5, -0.5 ], tex: [ 0.0, 1.0 ] },
            Vertex { pos: [  0.5,  0.5 ], tex: [ 1.0, 0.0 ] },
            Vertex { pos: [  0.5, -0.5 ], tex: [ 1.0, 1.0 ] },
            ];*/

        let verts = vec![
            Vertex { pos: [-1.0, -1.0], tex: [0.0, 0.0] },
            Vertex { pos: [-1.0,  1.0], tex: [0.0, 1.0] },
            Vertex { pos: [ 1.0,  1.0], tex: [1.0, 1.0] },
            Vertex { pos: [ 1.0, -1.0], tex: [1.0, 0.0] }
            ];
        
        let program = program!(display,
                               140 => { vertex: VERT_SRC,
                                        fragment: FRAG_SRC, } ).unwrap();
        let vbo = glium::vertex::VertexBuffer::new(display, &verts).unwrap().into_vertex_buffer_any();
        let img = ::image::open(&Path::new(&path)).unwrap();
        
        MapDrawer {
            vbo: vbo,
            program: program,
            tex: Texture2d::new(display,img).unwrap(),
        }
    }
    
    pub fn draw(
        &mut self,
        size: Vec2<f32>,
        player_pos: Vec2<usize>,
        transform: Mat4<f32>,
        target: &mut glium::Frame,
        ) {
                
        let uniforms = uniform! {
            transform: *transform.as_ref(),
            size: *size.as_ref(),
            sample: &self.tex,
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
}
