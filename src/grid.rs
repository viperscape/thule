use hex2d::{Coordinate};

#[derive(Debug)]
pub struct Grid {
    coords: Vec<Coordinate>,
}

impl Grid {
    pub fn new () -> Grid {
        let mut v = vec!();
        for x in 0..50 {
            for y in 0..50 {
                v.push(Coordinate::new(x, y));
            }
        }

        Grid { coords: v, }
    }
}
