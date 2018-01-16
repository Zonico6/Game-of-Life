extern crate serde;
extern crate serde_json;

use self::serde_json::{from_reader, Value};
use game_of_life_logic::CellSet;
use game_of_life_logic::Point;

use std::collections::BTreeSet;

type Clusters = Vec<Cluster>;
type Creatures = Vec<Creature>;
type Attributes = Vec<String>;

const DIR_EMPTY: u8 = 0b0000;
const DIR_UP: u8 =    0b0001;
const DIR_DOWN: u8 =  0b0010;
const DIR_RIGHT: u8 = 0b0100;
const DIR_LEFT: u8 =  0b1000;

const DIRECTION_PRIORITIES: [u8; 4] = [DIR_RIGHT, DIR_UP, DIR_LEFT, DIR_DOWN];

fn swap_value<T: Ord>(hash_set: &mut BTreeSet<T>, value: T) {
    if hash_set.contains(&value) {
        hash_set.remove(&value);
    } else {
        hash_set.insert(value);
    }
}

// TODO: Correct Iterator
/// Parses all the usable files in the provided directory and
/// converts the Json to in-memory creatures
pub fn make_creatures<T: Iterator>(file_paths: T) -> Creatures {
    for path in file_paths {
        unimplemented!()
    }
}

/// Compresses a given CellSet to a Creature object, which efficiently stores the cells and
/// is therefore most appropriate for converting to Json
///
/// Beginning at the top-left, cycling counter-clockwise
fn compress(mut cells: CellSet) -> (CellSet, Clusters) {
    while cells.len() != 0 {
        compress_individual(&mut cells, cells[0]); // FixMe: Indexing flaw
    }
}

/// Depending on what it encounters, it either moves the
/// cell into the CellContainer or compresses the Cluster
fn compress_individual(cells: &mut CellSet, cell: Point) -> CellContainer {
    let neighbours = neighbours(cells, cell);
    if neighbours == DIR_EMPTY {
        cells.remove(cell);
        CellContainer::Cell(cell)
    } else {
        let cluster = compress_cluster(cells, cell);
        cells.remove(cluster);
        cluster
    }
}

/// Returns upper left cell of this cluster so the compress_cluster method can start compressing
fn compression_starting_point(cells: &CellSet, mut cell: Point) -> Point {
    while cells.contains(Point {x: cell.x, y: cell.y + 1}) {
        cell.y += 1;
    }
    while cells.contains(Point {x: cell.x - 1, y: cell.y}) {
        cell.x -= 1;
    }
    cell
}

fn compress_cluster(cells: &CellSet, cell: Point) -> Cluster {
    let mut vertices = Vec![compression_starting_point(cells, cell)];
    let from_dir = Directions(DIR_RIGHT);
    loop {
        let (next, from_dir) = next_vertex(cells, vertices.last().unwrap(), from_dir);
        if next == vertices.first().unwrap() {
            vertices
        } else {
            vertices.push(next);
        }
    }
    let mt
    let mut cluster = Cluster {vertices, except};
}

/// Moves on to the next Vertex and returns it
fn next_vertex(cells: &CellSet, last_vert: Point, disabled_dirs: Directions) -> (Point, Directions) {
    let neighbours = neighbours(cells, cell);
    for dir in DIRECTION_PRIORITIES {
        if neighbours.contains(dir) {
            while cells.contains(dir.on_point(cell)) {
                cell.x -= dir.on_point(cell);
            }
            (cell, dir)
        }
    }
}

/// Returns all the directions, the cell have neighbours in
fn neighbours(cells: &CellSet, cell: Point) -> Directions {
    let mut dirs = Direction(0);
    if cells.contains(Point {x: cell.x + 1, y: cell.y}) {
        dirs.add(DIR_RIGHT);
    }
    if cells.contains(Point {x: cell.x, y: cell.y + 1}) {
        dirs.add(DIR_UP);
    }
    if cells.contains(Point {x: cell.x - 1, y: cell.y}) {
        dirs.add(DIR_LEFT);
    }
    if cells.contains(Point {x: cell.x, y: cell.y - 1}) {
        dirs.add(DIR_DOWN);
    }
    dirs
}

trait CellSetIntegrable {
    fn integrate(self, cell_set: &mut CellSet);
}

#[derive(Eq, PartialEq, Hash, Serialize, Deserialize, Debug)]
enum CellContainer {
    Cluster(Box<Cluster>),
    Cell(Point),
}

struct Directions(u8);
impl Directions {
    fn remove(&mut self, dirs: Directions) {
        self.0 &= !dirs.0
    }

    fn add(&mut self, dirs: Directions) {
        self.0 |= dirs.0
    }

    fn on_point(&self, mut point: Point) -> Point {
        if self.contains(DIR_RIGHT) {
            point = Point {x: point.x + 1, y: point.y}
        }
        if self.contains(DIR_LEFT) {
            point = Point {x: point.x - 1, y: point.y}
        }
        if self.contains(DIR_UP) {
            point = Point {x: point.x, y: point.y + 1}
        }
        if self.contains(DIR_DOWNs) {
            point = Point {x: point.x, y: point.y - 1}
        }
        point
    }

    fn contains(&self, dirs: Directions) -> bool {
        (self.0 & dirs.0) == dirs.0
    }
}

/// A vertex layer within a Cluster
struct ClusterLayer {
    height: usize,
    pos: isize,
    vertices: Vec<isize>,
}

impl ClusterLayer {
    fn new(vertices: Vec<isize>, height: usize, pos: isize) -> ClusterLayer {
        ClusterLayer {pos, height, vertices}
    }

    /// Integrate a section between two vertices of a ClusterLayer
    fn integrate_section(&self, cell_set: &mut CellSet, vertices: (isize, isize)) {
        for x in vertices.0 .. vertices.1 + 1 {
            for y in pos .. pos + height {
                cell_set.insert(Point {x, y});
            }
        }
    }
}

/// Parse the vertices from left to right and take turns between integrating a section
/// and preparing the integration of the next section
impl CellSetIntegrable for ClusterLayer {
    fn integrate(self, cell_set: &mut CellSet) {
        let mut last = None;
        for vert in &vertices {
            if let Some(last_vert) = last {
                self.integrate_section(last_vert, vert);
                last = None;
            } else {
                last = Some(vert);
            }
        }
    }
}

/// Essentially a more memory-efficient representation for a CellSet
/// Meant to be used when working with Json
#[derive(Eq, PartialEq, Hash, Serialize, Deserialize, Debug)]
struct Cluster {
    vertices: Vec<Point>,
    except: Vec<CellContainer>,
}

impl Cluster {
    fn new_complete(vertices: Vec<Point>) -> Cluster {
        Cluster {vertices, except: Vec::new()}
    }
    fn new(vertices: Vec<Point>, except: Vec<CellContainer>) -> Cluster {
        Cluster {vertices, except}
    }
}

/// Essentially moving up a creature's vertices and thus splitting up the creature
/// into multiple layers which then can be integrated with ease
impl CellSetIntegrable for Cluster {
    fn integrate(self, cell_set: &mut CellSet) {
        let mut sorted_y = self.vertices;
        sorted_y.sort_unstable_by(|&first, &second| {
            first.y.cmp(&second.y)
        });

        let mut vertex_table = BTreeSet::new();

        let mut last_vert = &sorted_y[0];
        for this_vert in sorted_y {
            swap_value(&mut vertex_table, this_vert);

            if last_vert.y != this_vert.y {
                let mut new_layer_verts: Vec<isize> = Vec::new();

                for vertex in &vertex_table {
                    new_layer_verts.push(vertex.x);
                }
                ClusterLayer::new(new_layer_verts, (this_vert.y - last_vert.y) as usize, last_vert.y).
                    integrate(cell_set);
            }

            last_vert = &this_vert;
        }

        let mut except_set = CellSet::new();

        for except in self.except {
            match except {
                CellContainer::Cluster(cluster) => {
                    cluster.integrate(&mut except_set);
                }
                CellContainer::Cell(cell) => {
                    except_set.insert(cell);
                }
            }
        }

        for ex_cell in except_set {
            cell_set.remove(&ex_cell);
        }
    }
}

/// A direct reflection of a Json-Creature
#[derive(Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct Creature {
    name: String,
    description: String,
    attributes: Attributes,
    cells: CellSet,
    clusters: Clusters,
    creatures: Creatures,
}

impl Creature {
    fn new(name: String, description: String, attributes: Attributes,
           cells: CellSet, creatures: Creatures) -> Creature {
        let (cells, clusters) = compress(cells);
        Creature {name, description, attributes, cells, clusters, creatures}
    }
}
impl CellSetIntegrable for Creature {
    fn integrate(self, cell_set: &mut CellSet) {
        for cell in self.cells {
            cell_set.insert(cell);
        }
        for cluster in self.clusters {
            cluster.integrate(cell_set);
        }
        for creature in self.creatures {
            creature.integrate(cell_set);
        }
    }
}