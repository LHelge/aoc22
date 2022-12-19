use std::{collections::{BTreeMap, HashSet}};



pub type Coord = (i64, i64);

#[derive(Debug)]
pub struct Sensor {
    pub position: Coord,
    pub beacon: Coord
}

#[derive(Debug)]
pub struct Cave {
    sensors: BTreeMap<Coord, u64>,
    beacons: HashSet<Coord>,
    top_left: Coord,
    bottom_right: Coord
}


fn manhattan_distance(one: &Coord, other: &Coord) -> u64 {
    one.0.abs_diff(other.0) + 
    one.1.abs_diff(other.1)
}

impl Cave {
    pub fn new(sensors: &Vec<Sensor>) -> Self {
        let mut cave = Self {
            sensors: BTreeMap::new(),
            beacons: HashSet::new(),
            top_left: (i64::MAX, i64::MAX),
            bottom_right: (i64::MIN, i64::MIN)
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

        cave
    }

    pub fn no_beacon_position_row(&self, y: i64) -> usize {
        let mut no_beacon: HashSet<i64> = HashSet::new();

        for (k, v) in self.sensors.iter() {
            let y_distance = y.abs_diff(k.1);

            if y_distance <= *v {
                for x_offset in 0..=v.abs_diff(y_distance) as i64 {
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

        no_beacon.len()
    }

    pub fn find_free_spot(&self, top_left: &Coord, bottom_right: &Coord) -> Option<Coord> {
        let stop_x = self.bottom_right.0.min(bottom_right.0);
        let stop_y = self.bottom_right.1.min(bottom_right.1);

        //println!("stop_x={}, stop_y={}", stop_x, stop_y);
        let mut pos = top_left.clone();

        loop {
            let mut found = false;
            //println!("Checking {:?}", pos);

            // Find sensors within range
            for (k, v) in self.sensors.iter() {
                let x_distance = pos.0.abs_diff(k.0);
                let y_distance = pos.1.abs_diff(k.1);
    
                // Check if within range
                //println!("  Checking beacon at {:?}. ({}+{}) < {}", k, x_distance, y_distance, v);
                if (x_distance + y_distance) <= *v  {
                    pos.0 = k.0 + (*v as i64 - y_distance as i64) + 1;
                    //println!("    Found becon at {:?}={}, advancing to {:?}", k, v, pos);
                    
                    if pos.0 > stop_x {
                        // Go to next line
                        pos.1 += 1;
                        pos.0 = top_left.0;

                        // Reached end No spot found
                        if pos.1 > stop_y {
                            return None;
                        }
                    }

                    found = true;
                    break;
                }

            }
            
            if found {
                continue;
            }
            return Some(pos);
        }
    }

}