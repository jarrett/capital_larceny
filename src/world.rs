use std::cell::RefCell;
use std::rc::Rc;

use tile;
use tile::{Chunk, Tile};
use tile::chunk;
use gen;
use gen::street::{STREET_WIDTH, BLOCK_WIDTH, GRID_SIZE};

pub struct World {
    pub chunks: Vec<Vec<Rc<RefCell<Chunk>>>>,
}

impl World {
    pub fn new(tile_program: &tile::Program) -> World {
        let num_chunks = (STREET_WIDTH + BLOCK_WIDTH) * GRID_SIZE / chunk::SIZE;
        let mut chunks = Vec::with_capacity(num_chunks);
        for y in 0..num_chunks {
            let mut row = Vec::with_capacity(num_chunks);
            for x in 0..num_chunks {
                let chunk = Chunk::blank(tile_program, x * chunk::SIZE, y * chunk::SIZE);
                row.push(Rc::new(RefCell::new(chunk)));
            }
            chunks.push(row);
        }
        let mut world = World { chunks: chunks };
        gen::generate(&mut world);
        world
    }
    
    pub fn chunk_containing(&self, x: usize, y: usize) -> &Rc<RefCell<Chunk>> {
        let chunk_x: usize = x / chunk::SIZE;
        let chunk_y: usize = y / chunk::SIZE;
        &self.chunks[chunk_y][chunk_x]
    }
    
    pub fn replace_tile(&mut self, x: usize, y: usize, tile: Tile) {
        let mut chunk = self.chunk_containing(x, y).borrow_mut().replace_tile(x, y, tile);
    }
}