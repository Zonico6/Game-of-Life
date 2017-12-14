extern crate serde;
extern crate serde_json;

use self::serde_json::{from_reader, Value};
use game_of_life_logic::CellSet;

use std::collections::HashSet;

type ClusterSet = HashSet<Cluster>;

trait CellSetIntegrable {
    fn integrate(&self, cell_set: &mut CellSet);
}

#[derive(Eq, PartialEq, Hash, Serialize, Deserialize, Debug)]
struct Cluster {
    x: i32,
    y: i32,
    x_ext: i16,
    y_ext: i16,
}

#[derive(Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct Creature {
    name: String,
    description: String,
    attributes: Vec<String>,
    cells: CellSet,
    clusters: ClusterSet,
}

impl CellSetIntegrable for Creature {
    fn integrate(&self, cell_set: &mut CellSet) {
        unimplemented!()
    }
}

impl CellSetIntegrable for CellSet {
    fn integrate(&self, cell_set: &mut CellSet) {
        unimplemented!()
    }
}

impl CellSetIntegrable for Cluster {
    fn integrate(&self, cell_set: &mut CellSet) {
        unimplemented!()
    }
}