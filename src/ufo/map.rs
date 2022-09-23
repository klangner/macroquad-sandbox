//! World map is grid based. Each grid contains information is unit can move over it
//! 

use std::fmt;


/// Map data
#[derive(Default, Clone)]
pub struct WorldMap {
    pub walkable : Vec<bool>,
    pub width : usize,
    pub height : usize,
}

impl WorldMap {

    /// Generates an empty map, consisting entirely of solid walls
    pub fn new(width: usize, height: usize) -> WorldMap {
        let map_tile_count = width*height;
        WorldMap {
            walkable : vec![true; map_tile_count],
            width,
            height,
        }
    }

    /// Generates map from vector data
    pub fn from_data(width: usize, height: usize, data: &[bool]) -> WorldMap {
        assert!(width*height == data.len());
        let walkable = data.to_owned();
        WorldMap {
            walkable,
            width,
            height,
        }
    }

    /// Get TileType at the given location
    pub fn is_walkable(&self, x: usize, y: usize) -> bool {
        if x >= self.width || y >= self.height {
            false
        } else {
            let idx = (y as usize) * self.width + (x as usize);
            self.walkable[idx]
        }
    }

    pub fn xy_idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x        
    }
}
    
impl fmt::Display for WorldMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            let bytes: Vec<u8> = (0..self.width)
                .map(|x| if self.is_walkable(x, y) {'#'} else {' '} as u8)
                .collect();
            let line = String::from_utf8(bytes).expect("Can't convert map to string");
            let _ = write!(f, "{}\n", line);
        }
        Ok(())
    }
}

/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_map() {
        let map = WorldMap::new(10, 10);
        for i in 0..10 {
            for j in 0..10 {
                assert!(map.is_walkable(i, j));
            }
        }
    }
}