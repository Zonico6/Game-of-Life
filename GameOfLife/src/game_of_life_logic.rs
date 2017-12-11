use std::collections::HashSet;

pub type CellSet = HashSet<Point>;

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

pub struct GameOfLifeLogic {
    next: Option<CellSet>,
    pub cells: CellSet
}
impl GameOfLifeLogic {
    pub fn new() -> GameOfLifeLogic {
        GameOfLifeLogic {next: None, cells: CellSet::new()}
    }

    pub fn tick(&mut self) {
        self.cells = CellSet::clone(self.next());
        self.next = None;
    }
    pub fn tick_times(&mut self, times: u16) {
        for i in 0..times {
            self.tick();
        }
    }

    pub fn next(&mut self) -> &CellSet {
        match self.next {
            Some(ref next) => &next,
            None => {
                // TODO: Optimize in readability
                let mut ret_set = CellSet::new();
                let mut covered = CellSet::new();

                for old_cell in &self.cells {
                    if self.survives(&old_cell) {
                        ret_set.insert(old_cell.clone());
                    }
                    for x in old_cell.x - 1 .. old_cell.x + 2 {
                        for y in old_cell.y - 1 .. old_cell.y + 2 {
                            covered.insert(old_cell.clone());

                            let p = Point {x, y};
                            if !covered.contains(&p) &&
                                !self.cells.contains(&p) &&
                                self.living_neighbours(&p) == 3 {

                                ret_set.insert(p.clone());
                            }
                        }
                    }
                }
                self.next = Some(ret_set);
                if let Some(ref ret) = self.next { ret } else { unreachable!() }
            }
        }
    }

    fn survives(&self, point: &Point) -> bool {
        match self.living_neighbours(point) {
            0 | 1 => false,
            2 | 3 => true,
            _ => false
        }
    }
    fn living_neighbours(&self, point: &Point) -> u8 {
        let mut neighbours = 0;
        for x in point.x-1 .. point.x+2 {
            for y in point.y-1 .. point.y+2 {
                let p = Point {x, y};
                if self.cells.contains(&p) && p != *point {
                    neighbours += 1;
                }
            }
        }
        neighbours
    }

    pub fn insert(&mut self, point: Point) {
        self.cells.insert(point);
    }
    // Yes, terrible naming design. No, I do not care.
    pub fn add(&mut self, x: i32, y: i32) {
        self.cells.insert(Point {x, y});
    }
    pub fn contains(&mut self, point: Point) -> bool {
        self.cells.contains(&point)
    }
}