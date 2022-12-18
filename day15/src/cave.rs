use std::{collections::{BTreeMap, BTreeSet, HashSet}, fmt::write};



pub type Coord = (i32, i32);

#[derive(Debug)]
pub struct Sensor {
    pub position: Coord,
    pub beacon: Coord
}

#[derive(Debug)]
pub struct Cave {
    sensors: BTreeMap<Coord, u32>,
    beacons: BTreeSet<Coord>,
    top_left: Coord,
    bottom_right: Coord
}


fn manhattan_distance(one: &Coord, other: &Coord) -> u32 {
    one.0.abs_diff(other.0) + 
    one.1.abs_diff(other.1)
}

impl Cave {
    pub fn new(sensors: &Vec<Sensor>) -> Self {
        let mut cave = Self {
            sensors: BTreeMap::new(),
            beacons: BTreeSet::new(),
            top_left: (i32::MAX, i32::MAX),
            bottom_right: (i32::MIN, i32::MIN)
        };

        for sensor in sensors {
            // Keep track of cave bounds
            cave.top_left.0 = cave.top_left.0.min(sensor.position.0).min(sensor.beacon.0);
            cave.top_left.1 = cave.top_left.1.min(sensor.position.1).min(sensor.beacon.1);
            cave.bottom_right.0 = cave.bottom_right.0.max(sensor.position.0).max(sensor.beacon.0);
            cave.bottom_right.1 = cave.bottom_right.1.max(sensor.position.1).max(sensor.beacon.1);

            cave.sensors.insert(sensor.position, manhattan_distance(&sensor.position, &sensor.beacon));
            cave.beacons.insert(sensor.beacon);
        }

        println!("Created cave from {:?} to {:?}", cave.top_left, cave.bottom_right);

        cave
    }

    pub fn no_beacon_position_row(&self, y: i32) -> u32 {
        let mut no_beacon: HashSet<i32> = HashSet::new();

        for (k, v) in self.sensors.iter() {
            let y_distance = y.abs_diff(k.1);

            if y_distance <= *v {
                for x_offset in 0..=v.abs_diff(y_distance) as i32 {
                    let x_left = k.0 - x_offset;
                    
                    if !self.beacons.contains(&(x_left,y)) {
                        no_beacon.insert(x_left);
                    }
                    
                    let x_right = k.0 + x_offset;
                    if !self.beacons.contains(&(x_right,y)) {
                        no_beacon.insert(x_right);
                    }
                }
            }
        }

        no_beacon.len() as u32
    }

}