#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

type ClusterSet = HashSet<Cluster>;

use serde_json::{Serialize, Deserialize, from_reader, Value};
use game_of_life_logic::CellSet;

trait CellSetIntegrable {
    fn integrate(&self, cell_set: &mut CellSet);
}

#[derive(Serialize, Deserialize, Debug)]
struct Cluster {
    x: i32,
    y: i32,
    x_ext: i16,
    y_ext: i16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Creature {
    name: String,
    description: String,
    attributes: Value,
    cells: CellSet,
    clusters: ClusterSet,
}

impl CellSetIntegrable for Creature {
    fn integrate(&self, cell_set: &mut CellSet) {
        !unimplemented!()
    }
}

impl CellSetIntegrable for CellSet {
    fn integrate(&self, cell_set: &mut CellSet) {
        !unimplemented!()
    }
}

impl CellSetIntegrable for Cluster {
    fn integrate(&self, cell_set: &mut CellSet) {
        !unimplemented!()
    }
}