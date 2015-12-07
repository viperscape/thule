use std::fs::File;
use std::sync::Arc;

use na::{Mat4,Vec2,Pnt3};

use glium::{self,Surface,Display};
use glium::vertex::VertexBufferAny;

use obj;
use genmesh;

use ::ui::{
    color,
};


static VERT_SRC: &'static str = r"
        #version 140

        in vec3 pos;
        in vec3 norm;

        uniform mat4  transform;
        uniform vec2  size;
        uniform vec3  color;

        out vec3 v_color;
        out vec3 v_position;
        out vec3 v_normal;

        void main() {
             v_position = pos * vec3(size,1.0);
             v_normal = norm;
             gl_Position = transform * vec4(v_position, 1.0);
             v_color = color;
        }
";

static FRAG_SRC: &'static str = r"
       #version 140
       in vec3 v_normal;
       in vec3 v_color;

       const vec3 LIGHT = vec3(-0.2, 0.8, 0.1);
       void main() {
            float lum = max(dot(normalize(v_normal), normalize(LIGHT)), 0.0);
            vec3 color = (0.3 + 0.7 * lum) * v_color;
            gl_FragColor = vec4(color, 1.0);
       }
";

#[derive(Copy,Clone)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub norm: [f32; 3],
    pub tex: [f32; 2],
}

pub struct MeshDrawer {
    pub verts: Arc<Vec<Vertex>>,
    vbo: glium::vertex::VertexBufferAny,
   // params: glium::DrawParameters,
    program: glium::Program,
}

impl MeshDrawer {
    pub fn new(verts : Vec<Vertex>,
               display: &Display) -> MeshDrawer {
        implement_vertex!(Vertex, pos, norm, tex);
        
       // let v = verts.iter().map(|vert| *Pnt3::from_array_ref(&vert.pos)).collect();

        let program = program!(display,
                               140 => { vertex: VERT_SRC,
                                        fragment: FRAG_SRC, } ).unwrap();
        let vbo = glium::vertex::VertexBuffer::new(display, &verts).unwrap().into_vertex_buffer_any();
        MeshDrawer {
            verts: Arc::new(verts),
            vbo: vbo,
           // params: params,
            program: program,
        }
    }

    pub fn draw(
        &mut self,
        size     : Vec2<f32>,
        color    : color::Color,
        transform: Mat4<f32>,
        target: &mut glium::Frame,
        ) {
        let uniforms = uniform! {
            transform: *transform.as_array(),
            size: *size.as_array(),
            color: color,
        };

        // draw parameters
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

        
        target.draw(&self.vbo,
                    &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                    &self.program, &uniforms, &params).unwrap();
    }
    
    pub fn new_from_path(path: &str, display: &Display) -> MeshDrawer {
        if let Some(f) = File::open(path).ok() {
            let mut data = ::std::io::BufReader::new(f);
            let data = obj::Obj::load(&mut data);
            
            let verts = load_wavefront(data);
            MeshDrawer::new(verts,display,)
        }
        else { panic!("mesh asset not found: {:?}", path); }
    }
}

/// to be rendered as a gl triangles list
// TODO: update to latest obj/genmesh, incorporating materials/groups
pub fn load_wavefront(data: obj::Obj<String>) -> Vec<Vertex> {
    let mut vertex_data = Vec::new();

    for shape in data.object_iter().next().unwrap().group_iter().flat_map(|g| g.indices().iter()) {
        match shape {
            &genmesh::Polygon::PolyTri(genmesh::Triangle { x: v1, y: v2, z: v3 }) => {
                for v in [v1, v2, v3].iter() {
                    let position = data.position()[v.0];
                    let texture = v.1.map(|index| data.texture()[index]);
                    let normal = v.2.map(|index| data.normal()[index]);

                    let texture = texture.unwrap_or([0.0, 0.0]);
                    let normal = normal.unwrap_or([0.0, 0.0, 0.0]);

                    vertex_data.push(Vertex {
                        pos: position,
                        norm: normal,
                        tex: texture,
                    })
                }
            },
            _ => unimplemented!()
        }
    }

    vertex_data
}
