use std::cell::RefCell;
use std::rc::Rc;

use self::Tile::*;
use self::WallTileType::*;
use self::FloorTileType::*;

// Tiles.

#[derive(Debug)]
pub enum Tile {
    WallTile(WallTileType),
    FloorTile(FloorTileType, OptOccupant)
}

impl Tile {
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Tile {
        match b {
            1 => { WallTile(BrickWall) }
            _ => { FloorTile(ConcreteFloor, None) }
        }
    }
    
    pub fn buffer(&self, positions: &mut Vec<f32>, colors: &mut Vec<f32>, indices: &mut Vec<u16>, x: u32, y: u32) {
        let o: u16 = positions.len() as u16 / 2;
        
        positions.push_all(&[
            x as f32 - 0.5, y as f32 - 0.5, // NW.
            x as f32 + 0.5, y as f32 - 0.5, // NE.
            x as f32 + 0.5, y as f32 + 0.5, // SE.
            x as f32 - 0.5, y as f32 + 0.5  // SW.
        ]);
        
        indices.push_all(&[
            o + 0, o + 1, o + 3, // NW, NE, SW.
            o + 1, o + 2, o + 3  // NE, SE, SW.
        ]);
        
        let (r, g, b) = match *self {
            WallTile(_)       => { (1.0, 1.0, 1.0) },
            FloorTile(_, _)   => { (0.0, 0.0, 0.0) }
        };
                
        for _ in 0u8..4u8 {
            colors.push_all(&[r, g, b]);
        }
    }
}

#[derive(Debug)]
pub enum WallTileType {
    BrickWall
}

#[derive(Debug)]
pub enum FloorTileType {
    ConcreteFloor
}

// Occupants of tiles.

type OptOccupant = Option<Rc<RefCell<Occupant>>>;

#[derive(Debug)]
pub enum Occupant {
    //MacGuffin
}