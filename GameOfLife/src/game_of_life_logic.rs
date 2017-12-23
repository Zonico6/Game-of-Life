use std::collections::HashSet;

pub type CellSet = HashSet<Point>;

#[derive(Serialize, Deserialize, Debug, Hash, Ord, PartialOrd, Clone, Eq, PartialEq)]
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

    /// Spawns the new cells around a given one
    fn spawn_cells(&self, ret_set: &mut CellSet, covered: &mut CellSet, cell: &Point) {
        for x in cell.x - 1 .. cell.x + 2 {
            for y in cell.y - 1 .. cell.y + 2 {
                let p = Point {x, y};

                covered.insert(p.clone());

                if !covered.contains(&p) &&
                    !self.cells.contains(&p) &&
                    self.living_neighbours(&p) == 3 {

                    ret_set.insert(p.clone());
                }
            }
        }
    }

    /// Generate the next generation of cells
    pub fn generate_next(&self) -> CellSet {
        let mut ret_set = CellSet::new();
        let mut covered = CellSet::new();

        for old_cell in &self.cells {
            // Check for survival
            if self.survives(&old_cell) {
                ret_set.insert(old_cell.clone());
            }
            self.spawn_cells(&mut ret_set, &mut covered, old_cell);
        }
        ret_set
    }

    /// Return the next generation of cells with regard to the next attribute
    pub fn next(&mut self) -> &CellSet {
        match self.next {
            Some(ref next) => &next,
            None => {
                self.next = Some(self.generate_next());
                if let Some(ref ret) = self.next { ret } else { unreachable!() }
            }
        }
    }

    /// Wrapper for GameOfLife::survives_set(..)
    fn survives(&self, cell: &Point) -> bool {
        GameOfLifeLogic::survives_set(&self.cells, cell)
    }
    /// Wrapper for GameOfLife::living_neighbours(..)
    fn living_neighbours(&self, point: &Point) -> u8 {
        GameOfLifeLogic::living_neighbours_in_set(&self.cells, point)
    }

    pub fn insert(&mut self, point: Point) {
        self.cells.insert(point);
    }
    // Yes, terrible naming. No, I do not care.
    pub fn add(&mut self, x: i32, y: i32) {
        self.cells.insert(Point {x, y});
    }
    pub fn contains(&mut self, point:  &Point) -> bool {
        self.cells.contains(point)
    }

    /// Tests for survival of a cell
    fn survives_set(cell_set: &CellSet, cell: &Point) -> bool {
        match GameOfLifeLogic::living_neighbours_in_set(cell_set, cell) {
            0 | 1 => false,
            2 | 3 => true,
            _ => false
        }
    }
    /// Returns number of neighbours of a cell
    fn living_neighbours_in_set(cell_set: &CellSet, point: &Point) -> u8 {
        let mut neighbours = 0;
        for x in point.x-1 .. point.x+2 {
            for y in point.y-1 .. point.y+2 {
                let p = Point {x, y};
                if cell_set.contains(&p) && p != *point {
                    neighbours += 1;
                }
            }
        }
        neighbours
    }
}