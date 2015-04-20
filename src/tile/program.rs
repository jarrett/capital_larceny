use std::path::Path;
use gl::types::*;

use glutil;

pub struct Program {
    pub id:             GLuint,
    
    pub model_view_idx: GLint,
    pub projection_idx: GLint,
    
    pub position_idx:   GLuint,
    pub color_idx:      GLuint
}

impl Program {
    pub fn new() -> Program {
        let id = glutil::make_program(&Path::new("glsl/tile.vert.glsl"), &Path::new("glsl/tile.frag.glsl"));
        Program {
            id:             id,
            
            model_view_idx: glutil::get_uniform_location(id, "model"),
            projection_idx: glutil::get_uniform_location(id, "projection"),
            
            position_idx:   glutil::get_attrib_location( id, "position"),
            color_idx:      glutil::get_attrib_location( id, "color")
        }
    }
}