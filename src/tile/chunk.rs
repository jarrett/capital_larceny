use std::ptr;
use std::mem;
use std::default::Default;
use image::{Pixel, RgbImage};
use gl;
use gl::types::*;
use libc::c_void;

use super::Tile;
use super::Program;
use camera::Camera;

pub const SIZE: usize = 20;

pub struct Chunk {
    min_x:           usize,
    min_y:           usize,
    
    tiles:           Vec<Tile>,
    
    vao:             GLuint,
    position_buffer: GLuint,
    color_buffer:    GLuint,
    index_buffer:    GLuint,
    
    index_count:     i32
}

// 32 x 32 tiles.
impl Chunk {
    pub fn new(program: &Program, min_x: usize, min_y: usize) -> Chunk {
        let mut chunk = Chunk {
            min_x: min_x, min_y: min_y,
            tiles: Vec::with_capacity(SIZE * SIZE),
            vao: 0, position_buffer: 0, color_buffer: 0, index_buffer: 0, index_count: 0
        };
        
        unsafe {
            gl::GenVertexArrays(1, &mut chunk.vao);
            gl::GenBuffers(1,      &mut chunk.position_buffer);
            gl::GenBuffers(1,      &mut chunk.color_buffer);
            gl::GenBuffers(1,      &mut chunk.index_buffer);
        }
        
        chunk.configure_vao(program);
        
        chunk
    }
    
    // Warning: This doesn't buffer to the GPU. Call buffer yourself on the returned chunk.
    pub fn blank(program: &Program, min_x: usize, min_y: usize) -> Chunk {
        let mut chunk = Chunk::new(program, min_x, min_y);
        
        for _ in 0..SIZE {
            for _ in 0..SIZE {
                chunk.tiles.push(Default::default());
            }
        }
        
        chunk
    }
    
    #[allow(dead_code)]
    pub fn from_image_buffer(
        program: &Program,
        chunk_min_x: usize, chunk_min_y: usize,
        image_buf: &RgbImage,
        img_min_x: usize, img_min_y: usize
    ) -> Chunk {
        let mut chunk = Chunk::new(program, chunk_min_x, chunk_min_y);
        
        let img_max_x = img_min_x + SIZE;
        let img_max_y = img_min_y + SIZE;
        for y in img_min_y..img_max_y {
          for x in img_min_x..img_max_x {
              let (r, g, b, _) = image_buf.get_pixel(x as u32, y as u32).channels4();
              let tile = Tile::from_rgb(r, g, b);
              chunk.tiles.push(tile);
          }
        }
        
        chunk.buffer();
        
        chunk
    }
    
    pub fn buffer(&mut self) {
        let chunk_size_sq = SIZE * SIZE;
        // 32 tiles x 32 tiles x 4 vertices x 2 floats.
        let mut positions: Vec<f32> = Vec::with_capacity(8 * chunk_size_sq);
        // 32 tiles x 32 tiles x 4 vertices x 3 floats.
        let mut colors: Vec<f32> = Vec::with_capacity(12 * chunk_size_sq);
        // 32 tiles x 32 tiles x 6 indices.
        let mut indices: Vec<u16> = Vec::with_capacity(6 * chunk_size_sq);
        
        for y in 0..SIZE {
            for x in 0..SIZE {
                let tile = &self.tiles[(y * SIZE + x)];
                tile.buffer(
                    &mut positions, &mut colors, &mut indices,
                    x + self.min_x, y + self.min_y
                );
            }
        }
        
        self.index_count = indices.len() as i32;
        
        unsafe {
          gl::BindBuffer(gl::ARRAY_BUFFER, self.position_buffer);
          gl::BufferData(
              gl::ARRAY_BUFFER,
              4 * positions.len() as i64,
              positions.as_ptr() as *const c_void,
              gl::DYNAMIC_DRAW
          );
          
          gl::BindBuffer(gl::ARRAY_BUFFER, self.color_buffer);
          gl::BufferData(
              gl::ARRAY_BUFFER,
              4 * colors.len() as i64,
              colors.as_ptr() as *const c_void,
              gl::DYNAMIC_DRAW
          );
          
          gl::BindBuffer(gl::ARRAY_BUFFER, 0);
          
          gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.index_buffer);
          gl::BufferData(
              gl::ELEMENT_ARRAY_BUFFER,
              2 * positions.len() as i64,
              indices.as_ptr() as *const c_void,
              gl::DYNAMIC_DRAW
          );
          gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }
    
    fn configure_vao(&mut self, program: &Program) {
        unsafe {
            gl::BindVertexArray(self.vao);
            
            gl::BindBuffer(gl::ARRAY_BUFFER, self.position_buffer);
            gl::EnableVertexAttribArray(program.position_idx);
            gl::VertexAttribPointer(program.position_idx, 2, gl::FLOAT, gl::FALSE, 0, ptr::null());
            
            gl::BindBuffer(gl::ARRAY_BUFFER, self.color_buffer);
            gl::EnableVertexAttribArray(program.color_idx);
            gl::VertexAttribPointer(program.color_idx, 3, gl::FLOAT, gl::FALSE, 0, ptr::null());
            
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
    }
    
    pub fn draw(&self, program: &Program, camera: &Camera) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.index_buffer);
            gl::UseProgram(program.id);
            gl::UniformMatrix4fv(program.model_view_idx, 1, gl::FALSE, mem::transmute(&camera.model_view));
            gl::UniformMatrix4fv(program.projection_idx, 1, gl::FALSE, mem::transmute(&camera.projection));
            gl::DrawElements(gl::TRIANGLES, self.index_count, gl::UNSIGNED_SHORT, ptr::null());
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
    }
    
    pub fn replace_tile(&mut self, x: usize, y: usize, tile: Tile) {
        let rel_x = x - self.min_x;
        let rel_y = y - self.min_y;
        self.tiles[rel_y * SIZE + rel_x] = tile;
    }
}