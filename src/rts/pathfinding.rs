
use pathfinding::prelude::{absdiff, astar};
use crate::rts::WorldMap;



pub fn find_path(map: &WorldMap, from: (i32, i32), to: (i32, i32))  -> Vec<(i32, i32)> {
    let result = astar(
        &from, 
        |&(x, y)| map.get_available_exits(x as usize, y as usize).into_iter().map(|p| ((p.0 as i32, p.1 as i32), 1)).collect::<Vec<((i32, i32), i32)>>(), 
        |&(x, y)| (absdiff(x, to.0) + absdiff(y, to.1)),
        |p| *p == to);
    result.unwrap_or((vec![], 0)).0
}