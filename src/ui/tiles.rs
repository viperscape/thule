use std::fs::File;
use std::sync::Arc;

use na::{Mat4,Vec3};

use glium::{self,Surface,Display};
use glium::vertex::VertexBufferAny;

use obj;

use ::ui::mesh::{Vertex,load_wavefront};


static VERT_SRC: &'static str = r"
        #version 140

        in vec3 pos;
        in vec3 norm;

        in vec3 pos_tile;
        in vec3 pos_player;
        in vec3 color;
        in int visible;

        in vec4 heights; // w for tile center vertex
        in vec3 heights_too; // second set of height averages for neighbors

        in vec3 color_fog;

        uniform mat4  pv;
        //uniform mat4  m;
        uniform vec3  size;
        

        out vec4 v_color;
        out vec3 v_position;
        out vec3 v_normal;

        void main() {
          vec3 colors = vec3(0.0,0.0,0.0);
             if (visible == 1) {
               vec3 off = vec3(0.0,0.0,0.0);
               if ((gl_VertexID == 1) ||
                   (gl_VertexID == 3) ||
                   (gl_VertexID == 6) ||
                   (gl_VertexID == 11) ||
                   (gl_VertexID == 13) ||
                   (gl_VertexID == 17)) 
                { off = vec3(0.0,heights.w * -1.0,0.0); }

              else if ((gl_VertexID == 5) ||
                   (gl_VertexID == 9)) 
                { off = vec3(0.0,heights.x,0.0); }
              else if ((gl_VertexID == 15) ||
                   (gl_VertexID == 10)) 
                { off = vec3(0.0,heights.y,0.0); }
              else if ((gl_VertexID == 14) ||
                   (gl_VertexID == 16)) 
                { off = vec3(0.0,heights.z,0.0); }

              else if ((gl_VertexID == 0) ||
                   (gl_VertexID == 4))
                { off = vec3(0.0,heights_too.x,0.0); colors = vec3(1.0,0.0,0.0); }
              else if ((gl_VertexID == 2) ||
                   (gl_VertexID == 8)) 
                { off = vec3(0.0,heights_too.y,0.0); colors = vec3(0.0,1.0,0.0); }
              else if ((gl_VertexID == 12) ||
                   (gl_VertexID == 7)) 
                { off = vec3(0.0,heights_too.z,0.0); colors = vec3(0.0,0.0,1.0); }


               v_position = (pos + off) * size;
             }
             else { v_position = vec3(-3000.0,-3000.0,-3000.0); }

             vec3 apos = v_position + pos_tile;
             v_normal = norm;
             gl_Position = pv * vec4(apos, 1.0);

//distance of fragment in worldspace
// TODO: move to frag
float distx = apos.x-pos_player.x;
float distz = apos.z-pos_player.z;
float dist = sqrt(pow(distx,2.0)+pow(distz,2.0));

float fog_start = 1300 * (size.x / 100.); // this removes zoom
float fog_end = 1900 * (size.x / 100.);

//linear interpolation
float fog_factor = (dist-fog_start)/(fog_end-fog_start);
fog_factor = clamp(fog_factor,0,1);

          //v_color = vec4(color,1.0);
          v_color = vec4(mix(colors,color_fog,vec3(fog_factor)),1.0);
        }
";

static FRAG_SRC: &'static str = r"
       #version 140
       in vec3 v_normal;
       in vec4 v_color;

       const vec3 LIGHT = vec3(-0.2, 0.8, 0.1);
       void main() {
            float lum = max(dot(normalize(v_normal), normalize(LIGHT)), 0.0);
            vec4 color = (0.3 + 0.7 * lum) * v_color;
            gl_FragColor = color;
       }
";

#[derive(Copy, Clone)]
pub struct Attr {
    pub pos_tile: (f32,f32,f32),
    pub pos_player: (f32,f32,f32),
    pub color: (f32,f32,f32),
    pub color_fog: (f32,f32,f32),
    pub visible: i32,
    pub heights: (f32,f32,f32,f32),
    pub heights_too: (f32,f32,f32),
}

// TODO: consider using MeshDrawer with traits instead of reimpl
pub struct TileDrawer {
    pub verts: Arc<Vec<Vertex>>,
    vbo: glium::vertex::VertexBufferAny,
    program: glium::Program,
    pub inst: glium::vertex::VertexBuffer<Attr>,
}

impl TileDrawer {
    pub fn new(verts : Vec<Vertex>,
               display: &Display) -> TileDrawer {
        //implement_vertex!(Vertex, pos, norm, tex);
        
        let program = program!(display,
                               140 => { vertex: VERT_SRC,
                                        fragment: FRAG_SRC, } ).expect("unable to build tile drawer program");
        let vbo = glium::vertex::VertexBuffer::new(display, &verts).expect("unable to buld tile drawer vbo").into_vertex_buffer_any();

        let tile_inst = {
            implement_vertex!(Attr,
                              pos_tile,
                              pos_player,
                              color,
                              color_fog,
                              visible,
                              heights,heights_too);

            let data = vec![
                Attr {
                    pos_tile: (0.,0.,0.),
                    pos_player: (0.,0.,0.),
                    color: (1.,1.,1.),
                    visible: 1,
                    color_fog: (1.,1.,1.),
                    heights: (0.,0.,0.,0.),
                    heights_too: (0.,0.,0.),
                }
                ;(::GRIDSIZE * ::GRIDSIZE)];

            glium::vertex::VertexBuffer::dynamic(display, &data).expect("unable to build tile drawer attr inst vbo")
        };
        
        TileDrawer {
            verts: Arc::new(verts),
            vbo: vbo,
            program: program,
            inst: tile_inst,//.into_vertex_buffer_any(),
        }
    }

    pub fn draw(
        &mut self,
        size: Vec3<f32>,
        pv: Mat4<f32>, // persp*view mat
        //m: Mat4<f32>, // model mat
        target: &mut glium::Frame,
        ) {
        let uniforms = uniform! {
            pv: *pv.as_ref(),
            //m: *m.as_array(),
            size: *size.as_ref(),
        };

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

        
        target.draw((&self.vbo,self.inst.per_instance().expect("unable to instance tile drawer")),
                    &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                    &self.program, &uniforms, &params).expect("tile drawer target draw failure");
    }
    
    pub fn new_from_path(path: &str, display: &Display) -> TileDrawer {
        if let Some(f) = File::open(path).ok() {
            let mut data = ::std::io::BufReader::new(f);
            let data = obj::Obj::load(&mut data);
            
            let verts = load_wavefront(data);
            TileDrawer::new(verts,display,)
        }
        else { panic!("mesh asset not found: {:?}", path); }
    }
}
