use std::io::{prelude::*, stdin, StdinLock};
use super::*;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
struct Observation {
    units: Units,
    units_mask: Vec<Vec<bool>>,  // whether the unit exists and is visible to you. units_mask[t][i] is whether team t's unit i can be seen and exists.
    sensor_mask: Vec<Vec<bool>>,  // whether the tile is visible to the unit for that team
    map_features: MapFeatures,
    relic_nodes_mask: Vec<bool>, // whether the relic node exists and is visible to you
    relic_nodes: Vec<[i32; 2]>,  // position of the relic nodes
    team_points: Vec<u32>,  // points scored by each team in the current match
    team_wins: Vec<u8>, // number of wins each team has in the current game/episode
    steps: u32,  // number of steps taken in the current game/episode
    match_steps: u32  // number of steps taken in the current match
}

#[derive(Debug, Serialize, Deserialize)]
struct Units {
    position: Vec<Vec<[i32; 2]>>,
    energy: Vec<Vec<i32>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct MapFeatures {
    energy: Vec<Vec<i32>>,  // amount of energy on the tile
    tile_type: Vec<Vec<i32>>,  // type of the tile. 0 is empty, 1 is a nebula tile, 2 is asteroid
}


#[derive(Debug, Serialize, Deserialize)]
struct EnvCfg {
    max_units: Option<u32>,
    match_count_per_episode: Option<u8>,
    max_steps_in_match: Option<u32>,
    map_height: Option<u16>,
    map_width: Option<u16>,
    num_teams: Option<u8>,
    unit_move_cost: Option<u16>,
    unit_sap_cost: Option<u16>,
    unit_sap_range: Option<u16>,
    unit_sensor_range: Option<u16>,
}

#[derive(Debug, Serialize, Deserialize)]
struct EnvInfo {
    obs: Observation,
    step: u32,
    remainingOverageTime: u32, // total amount of time your bot can use whenever it exceeds 2s in a turn
    player: String, // your player id
    info: EnvCfg,
}


/// Represents Action performed by Agent
pub type Action = String;


/// Environment wrapper to interact with Lux AI API I/O
pub struct Environment {
    actions: Vec<Action>,
    curr_line: String,
    stdin: StdinLock<'static>,
}

impl Environment {
    /// Initializes Environment with stdout stdin
    ///
    /// # Parameters
    ///
    /// None
    ///
    /// # Returns
    ///
    /// A new created `Environment`
    pub fn new() -> Self {
        Self {
            actions: vec![],
            curr_line: String::new(),
            stdin: stdin().lock(),
        }
    }

    fn read_line(&mut self) {
        self.curr_line.clear();
        self.stdin.read_line(&mut self.curr_line);
        eprintln!("RUST got input: {}", self.curr_line);
    }

    pub fn read_env(&mut self) {
        self.read_line();
        let deserialized: EnvInfo = serde_json::from_str(&self.curr_line).unwrap();
        eprintln!("deserialized = {:?}", deserialized);
    }

}
