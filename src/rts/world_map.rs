//! World map is grid based. Each grid contains information is unit can move over it
//! 

use std::fmt;


#[derive(PartialEq, Copy, Clone, Debug, Eq, Hash)]
pub struct Tile {
    pub is_blocked: bool,
}

/// Map data
#[derive(Default, Clone)]
pub struct WorldMap {
    pub tiles : Vec<Tile>,
    pub width : usize,
    pub height : usize,
}

impl Tile {
    pub fn new(is_blocked: bool) -> Tile {
        Tile { is_blocked }
    }

    pub fn walkable() -> Tile {
        Tile { is_blocked: false }
    }

    pub fn blacked() -> Tile {
        Tile { is_blocked: true }
    }
}

impl WorldMap {

    /// Generates an empty map, consisting entirely of solid walls
    pub fn new(width: usize, height: usize) -> WorldMap {
        let map_tile_count = width*height;
        WorldMap {
            tiles : vec![Tile::new(true); map_tile_count],
            width,
            height,
        }
    }

    /// Create map from given string
    pub fn from_string(map_string: &str) -> WorldMap {
        let lines: Vec<&str> = map_string.split("\n")
            .map(|l| l.trim())
            .filter(|l| l.len() > 0)
            .collect();
        let cols = lines.iter().map(|l| l.len()).max().get_or_insert(1).to_owned();
        let rows = lines.len();
        let mut map = WorldMap::new(cols, rows);

        for i in 0..rows {
            let line = lines[i].as_bytes();
            for j in 0..line.len() {
                if line[j] as char == ' ' {
                    map.set_tile(j, i, Tile::walkable());
                }
            }
        }
        map
    }

    /// Get TileType at the given location
    pub fn at(&self, x: usize, y: usize) -> Tile {
        if x >= self.width || y >= self.height {
            Tile::blacked()
        } else {
            let idx = (y as usize) * self.width + (x as usize);
            self.tiles[idx]
        }
    }

    /// Get available exists from the given tile
    pub fn get_available_exits(&self, x: usize, y: usize) -> Vec<(usize, usize, f32)> {
        let mut exits = Vec::new();

        // Cardinal directions
        if x > 0 && self.is_exit_valid(x-1, y) { exits.push((x-1, y, 1.0)) };
        if self.is_exit_valid(x+1, y) { exits.push((x+1, y, 1.0)) };
        if y > 0 && self.is_exit_valid(x, y-1) { exits.push((x, y-1, 1.0)) };
        if self.is_exit_valid(x, y+1) { exits.push((x, y+1, 1.0)) };

        // Diagonals
        if x > 0 && y > 0 && self.is_exit_valid(x-1, y-1) { exits.push((x-1, y-1, 1.45)); }
        if y > 0 && self.is_exit_valid(x+1, y-1) { exits.push((x+1, y-1, 1.45)); }
        if x > 0 && self.is_exit_valid(x-1, y+1) { exits.push((x-1, y+1, 1.45)); }
        if self.is_exit_valid(x+1, y+1) { exits.push((x+1, y+1, 1.45)); }

        exits
    }    
 
    // Check if given tile can be accessed
    fn is_exit_valid(&self, x:usize, y:usize) -> bool {
        self.at(x, y).is_blocked == false
    }

    /// Modify tile at the given location
    pub fn set_tile(&mut self, x: usize, y: usize, tile: Tile) {
        if x < self.width && y < self.height {
            let idx = self.xy_idx(x as usize, y as usize);
            self.tiles[idx] = tile;
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
                .map(|x| if self.at(x, y).is_blocked {'#'} else {' '} as u8)
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
                assert!(map.at(i, j).is_blocked);
            }
        }
    }

    #[test]
    fn test_from_string() {
        let map_str = "
        ##########
        #        #
        ##########
        ";
        let map = WorldMap::from_string(map_str);

        assert_eq!(map.width, 10);
        assert_eq!(map.height, 3);
        for i in 0..10 {
            assert!(map.at(i, 0).is_blocked);
            assert!(map.at(i, 2).is_blocked);
            if i == 0 || i == 9 {
                assert!(map.at(i, 1).is_blocked);
            } else {
                assert!(map.at(i, 1).is_blocked == false);
            }
        }
    }
}