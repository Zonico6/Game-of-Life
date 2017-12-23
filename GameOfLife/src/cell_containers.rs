extern crate serde;
extern crate serde_json;

use self::serde_json::{from_reader, Value};
use game_of_life_logic::CellSet;
use game_of_life_logic::Point;

use std::collections::BTreeSet;

type Clusters = Vec<Cluster>;
type Creatures = Vec<Creature>;
type Attributes = Vec<String>;

type CellRect = (Point, Point);

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
pub fn make_creatures(file_paths: Vec<isize>) -> Creatures {
    for path in file_paths {
        unimplemented!()
    }
}

/// Compresses a given CellSet to a Creature object, which efficiently stores the cells and
/// is therefore most appropriate for converting to Json
///
/// Beginning at the top, cycling counter-clockwise
fn compress(cells: CellSet) -> (CellSet, Clusters) {
    unimplemented!();
}

trait CellSetIntegrable {
    fn integrate(self, cell_set: &mut CellSet);
}

#[derive(Eq, PartialEq, Hash, Serialize, Deserialize, Debug)]
enum CellContainer {
    Cluster(Box<Cluster>),
    Cell(Point),
}

/// A vertex layer within a Cluster
struct ClusterLayer {
    height: u16,
    pos: isize,
    vertices: Vec<isize>,
}

impl ClusterLayer {
    fn new(vertices: Vec<isize>, height: u16, pos: isize) -> ClusterLayer {
        ClusterLayer {pos, height, vertices}
    }
}
impl CellSetIntegrable for ClusterLayer {
    fn integrate(self, cell_set: &mut CellSet) {
        unimplemented!();
    }
}

/// Essentially a more memory-efficient representation for a CellSet
/// Meant to be used when working with Json
#[derive(Eq, PartialEq, Hash, Serialize, Deserialize, Debug)]
struct Cluster {
    vertices: Vec<Point>,
    except: Vec<CellContainer>,
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
                ClusterLayer::new(new_layer_verts, (this_vert.y - last_vert.y) as u16, last_vert.y).
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