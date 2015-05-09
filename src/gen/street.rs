use std::cell::RefCell;
use std::rc::Rc;

use super::fill_rect;
use tile::Tile::FloorTile;
use tile::FloorTileType::{GrassFloor, AsphaltFloor};
use world::World;

pub const GRID_SIZE:    usize = 10;  // How many streets wide and tall the city is.
pub const STREET_WIDTH: usize = 15;  // How many tiles wide a street is.
pub const BLOCK_WIDTH:  usize = 185; // How many tiles wide a city block is, not counting any adjacent streets.

pub struct Grid {
    pub intersections: Vec<Vec<Rc<RefCell<Intersection>>>>
}

pub type Connection = Option<Rc<RefCell<Street>>>;

pub struct Intersection {
    // x and y are in grid coords, not world coords.
    pub x:  usize,
    pub y:  usize,
    pub n:  Connection,
    pub ne: Connection,
    pub e:  Connection,
    pub se: Connection,
    pub s:  Connection,
    pub sw: Connection,
    pub w:  Connection,
    pub nw: Connection
}

pub enum ConnectDir { N, NE, E, SE, S, SW, W, NW }

pub struct Street {
    pub inter1:     Rc<RefCell<Intersection>>,
    pub inter1_dir: ConnectDir,
    pub inter2:     Rc<RefCell<Intersection>>,
    pub inter2_dir: ConnectDir
}

impl Grid {
    pub fn generate() -> Grid {
        // Initialize all the intersections. They're not connected by streets yet.
        let mut grid = Grid { intersections: Vec::with_capacity(GRID_SIZE) };
        for y in 0..GRID_SIZE {
            let mut row = Vec::with_capacity(GRID_SIZE);
            for x in 0..GRID_SIZE {
                let inter = Intersection::new(x, y);
                row.push(Rc::new(RefCell::new(inter)));
            }
            grid.intersections.push(row);
        }
        
        for (y, row) in grid.intersections.iter().enumerate() {
            for (x, inter1) in row.iter().enumerate() {
                if x < GRID_SIZE - 1 {
                    let inter2 = &grid.intersections[y][x + 1];
                    let street = Rc::new(RefCell::new(Street {
                        inter1:     inter1.clone(),
                        inter1_dir: ConnectDir::E,
                        inter2:     inter2.clone(),
                        inter2_dir: ConnectDir::W
                    }));
                    inter1.borrow_mut().e = Some(street.clone());
                    inter2.borrow_mut().w = Some(street);
                }
                if y < GRID_SIZE - 1 {
                    let inter2 = &grid.intersections[y + 1][x];
                    let street = Rc::new(RefCell::new(Street {
                        inter1:     inter1.clone(),
                        inter1_dir: ConnectDir::S,
                        inter2:     inter2.clone(),
                        inter2_dir: ConnectDir::N
                    }));
                    inter1.borrow_mut().s = Some(street.clone());
                    inter2.borrow_mut().n = Some(street);
                }
            }
        }
        
        grid
    }
    
    pub fn rasterize(&self, world: &mut World) {
        for row in self.intersections.iter() {
            for inter in row.iter() {
                inter.borrow().rasterize(world);
            }
        }
    }
}

impl Intersection {
    pub fn new(x: usize, y: usize) -> Intersection {
        Intersection {
            x: x, y: y,
            n: None, ne: None, e: None, se: None,
            s: None, sw: None, w: None, nw: None
        }
    }
    
    // In world coords.
    fn max_x(&self) -> usize {
        self.x * (STREET_WIDTH + BLOCK_WIDTH) + STREET_WIDTH - 1
    }
    
    // In world coords.
    fn max_y(&self) -> usize {
        self.y * (STREET_WIDTH + BLOCK_WIDTH) + STREET_WIDTH - 1
    }
    
    // In world coords.
    fn min_x(&self) -> usize {
        self.x * (STREET_WIDTH + BLOCK_WIDTH)
    }
    
    // In world coords.
    fn min_y(&self) -> usize {
        self.y * (STREET_WIDTH + BLOCK_WIDTH)
    }
    
    pub fn rasterize(&self, world: &mut World) {
        // Fill the intersection with asphalt.
        fill_rect(
            world,
            FloorTile(AsphaltFloor, None),
            self.min_x(), self.min_y(),
            self.max_x(), self.max_y()
        );
        
        if self.x < GRID_SIZE - 1 {
          // Fill the east street with asphalt.
          fill_rect(
              world,
              FloorTile(AsphaltFloor, None),
              self.max_x() + 1, self.min_y(),
              self.max_x() + BLOCK_WIDTH, self.max_y()
          );
        }
        
        if self.y < GRID_SIZE - 1 {
            // Fill the south street with asphalt.
            fill_rect(
                world,
                FloorTile(AsphaltFloor, None),
                self.min_x(), self.max_y() + 1,
                self.max_x(), self.max_y() + BLOCK_WIDTH
            );
        }
    }
}