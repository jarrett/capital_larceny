use std::cell::RefCell;
use std::rc::Rc;
use std::default::Default;

use self::Tile::*;
use self::WallTileType::*;
use self::FloorTileType::*;

// Tiles.

#[derive(Debug, Clone)]
pub enum Tile {
    WallTile(WallTileType),
    FloorTile(FloorTileType, OptOccupant)
}

#[derive(Debug, Clone)]
pub enum WallTileType {
    BrickWall
}

#[derive(Debug, Clone)]
pub enum FloorTileType {
    ConcreteFloor,
    AsphaltFloor,
    GrassFloor
}

// Occupants of tiles.

type OptOccupant = Option<Rc<RefCell<Occupant>>>;

#[derive(Debug)]
pub enum Occupant {
    //MacGuffin
}

impl Tile {
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Tile {
        match (r, g, b) {
            (255, 255, 255) => { WallTile(BrickWall) },
            (0,   0,   0  ) => { FloorTile(AsphaltFloor, None) },
            (128, 128, 128) => { FloorTile(ConcreteFloor, None) },
            _               => { FloorTile(GrassFloor, None) }
        }
    }
    
    pub fn buffer(&self, positions: &mut Vec<f32>, colors: &mut Vec<f32>, indices: &mut Vec<u16>, x: usize, y: usize) {
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
            WallTile(_)                 => { (1.00,  1.00,  1.00) },
            FloorTile(AsphaltFloor, _)  => { (0.10,  0.10,  0.10) },
            FloorTile(GrassFloor, _)    => { (0.09,  0.37,  0.18) }
            FloorTile(ConcreteFloor, _) => { (0.50,  0.50,  0.50) }
        };
                
        for _ in 0u8..4u8 {
            colors.push_all(&[r, g, b]);
        }
    }
}

impl Default for Tile {
    fn default() -> Tile { FloorTile(GrassFloor, None) }
}