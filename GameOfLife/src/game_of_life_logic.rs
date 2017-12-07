type PointSet = HashSet<Point>;

struct Point {
    x: i32,
    y: i32
}
struct GameOfLifeLogic {
    next: Option<PointSet>,
    pub cells: PointSet
}
impl GameOfLifeLogic {
    pub fn tick(&mut self) {
        self.cells = next();
        next = None;
    }

    pub fn next(&self) -> PointSet {
        match self.next {
            Some(n) => n,
            None => {
                // TODO: Optimize in readability
                let mut ret_set = PointSet::new();
                let mut covered = PointSet::new();
                for old_cell in self.cells {
                    if survives(old_cell) {
                        ret_set.insert(old_cell);
                    }
                    for x in old_cell.x - 1 .. old_cell.x + 1 {
                        for y in self.cells.y - 1 .. self.cells.y + 1 {
                            let p = Point(x, y);

                            if !covered.contains(p) && self.cells.contains(Point { x, y }) && livingNeighbours(p) == 3 {
                                retSet.insert(p)
                            }
                        }
                    }
                }
            }
        }
    }

    fn survives(&self, point: &Point) -> bool {
        match living_neighbours(point) {
            0 | 1 => false,
            2 | 3 => true,
            _ => false
        }
    }

    fn living_neighbours(&self, point: &Point) -> u8 {
        let mut neighbours = 0;
        for x in point.x-1 .. point.x+1 {
            for y in point.y-1 .. point.y+1 {
                let p = Point {x, y};
                if self.cells.contains(p) && point != p {
                    neighbours += 1;
                }
            }
        }
        neighbours
    }

    // TODO: Maybe solve this more nicely by casting this class to PointSet
    pub fn insert(&mut self, point: Point) {
        self.cells.insert(point);
    }
    pub fn contains(&mut self, point: Point) -> bool {
        self.cells.contains(point)
    }
}