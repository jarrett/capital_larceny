pub mod street;

use std::rc::Rc;
use std::cell::RefCell;

use tile::{chunk, Chunk, Tile};
use world::World;

pub fn generate(world: &mut World) {    
    street::Grid::generate().rasterize(world);
    
    for row in world.chunks.iter() {
        for chunk in row.iter() {
            chunk.borrow_mut().buffer();
        }
    }
}

// Fills from min to max inclusive.
pub fn fill_rect(
    world: &mut World, tile: Tile,
    min_x: usize, min_y: usize, max_x: usize, max_y: usize
) {
    for y in min_y..(max_y + 1) {
        for x in min_x..(max_x + 1) {
            world.replace_tile(x, y, tile.clone());
        }
    }
}