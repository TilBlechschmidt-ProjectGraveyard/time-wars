#![allow(dead_code)]
use std::collections::BTreeMap;

#[cfg(feature = "f64-precision")]
type Coordinate = (f64, f64);
#[cfg(not(feature = "f64-precision"))]
type Coordinates = (f32, f32);
type Orientation = f32;
type Keyframe = Vec<(ID, Coordinates, Orientation)>;
type TimeIndex = usize;
type Player = usize;
type ID = usize;


// ------------------------------------------ PORTAL -----------------------------------------

struct Endpoint {
    location: Coordinates,
    creation: TimeIndex,
    expiration: TimeIndex,
    scale: f32
}

struct Portal {
    player: Player,
    origin: Endpoint,
    dest: Endpoint,
    compression_factor: (f32, f32) // (size, time) - compression level when traveling origin->dest
}

// -------------------------------------------- AI -------------------------------------------

#[derive(PartialEq)]
enum AIType {
    Scout,
    Knight
}

struct AI {
    ai_type: AIType,
    player: Player,
    start_location: Coordinates,
    start_orientation: Orientation
}

// ------------------------------------------ SERVER -----------------------------------------

pub struct Server {
    portals: Vec<Portal>,
    keyframes: BTreeMap<TimeIndex, Keyframe>,
    ais: Vec<AI>
}

impl Server {
    pub fn new() -> Server {
        if cfg!(feature = "f64-precision") { println!("Using double precision floating mode!") }
        Server {
            portals: Vec::new(),
            keyframes: BTreeMap::new(),
            ais: Vec::new()
        }
    }

    fn get_ai(&self, id: ID) -> &AI {
        &self.ais[id]
    }

    fn get_closest_keyframe(&self, target: TimeIndex) -> (usize, Keyframe) {
        let x = self.keyframes.iter().rev().find(|&(key, _)| *key <= target).unwrap();
        (*x.0, x.1.clone())
    }

    fn calculate(&mut self, target: TimeIndex) -> Keyframe { //TODO: Don't use .clone() all over the place and use pointers instead
        let closest = self.get_closest_keyframe(target);
        if closest.0 == target { return closest.1 };

        let mut current: TimeIndex = closest.0;
        let mut last = closest.1;
        while current != target {
            current = current + 1;
            let mut ais = Vec::with_capacity(last.len());
            for x in last.iter() {
                let id = x.0;
                let mut loc = x.1;
                let o = x.2;
                let ai = self.get_ai(id);

                if ai.ai_type == AIType::Scout {
                    loc.0 = loc.0 + 1.0;
                    loc.1 = loc.0.powf(2.0);
                }

                ais.push((id, loc, o));
            }
            self.keyframes.insert(current, ais.clone());
            last = ais;
        }
        self.keyframes.get(&target).unwrap().clone()
    }

    fn print_keyframes(&self) {
        for keyframe in self.keyframes.iter() {
            for ai in keyframe.1.iter() {
                println!("AI: {}, X: {}, Y: {}", ai.0, (ai.1).0, (ai.1).1);
            }
        }
    }

    pub fn start_game(&mut self) {
        println!("A new game has been started!");
    }
}

#[test]
fn calculate_keyframes() {
    let mut s = Server::new();
    let ai = AI {
        ai_type: AIType::Scout,
        player: 0,
        start_location: (0.0, 0.0),
        start_orientation: 0.0
    };
    s.ais.push(ai);
    s.keyframes.insert(0, vec![(0, (0.0, 0.0), 0.0)]);

    s.calculate(10);
    {
        let loc = s.keyframes.get(&(10 as usize)).unwrap()[0].1;
        assert_eq!(loc, (10.0, 100.0));
    }
    s.calculate(20);
    {
        let loc = s.keyframes.get(&(20 as usize)).unwrap()[0].1;
        assert_eq!(loc, (20.0, 400.0));
    }
}
