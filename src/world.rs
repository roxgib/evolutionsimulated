use rand::seq::SliceRandom;

use crate::config::get_config;

use super::creature::Creature;
use super::gene::{EyeColour, SkinColour};

pub struct World {
    pub creatures: Vec<Creature>,
}

impl World {
    pub fn new() -> World {
        World {
            creatures: Vec::new(),
        }
    }

    pub fn spawn_random_organisms(&mut self, count: usize) {
        for _ in 0..count {
            self.creatures.push(Creature::new_random());
        }
    }

    pub fn tick(&mut self) {
        for creature in &mut self.creatures {
            creature.tick();
        }
        self.creatures.retain(|creature| creature.age < get_config().lifespan + 15);
        if self.creatures.len() < get_config().max_creatures {
            self.reproduce();
        }
    }

    fn reproduce(&mut self) {
        let mut new_creatures = Vec::new();
        self.creatures.shuffle(&mut rand::thread_rng());
        for i in 0..self.creatures.len() {
            if new_creatures.len() >= self.creatures.len() / 20{
                break;
            }
            if self.creatures[i].last_reproduced < 15 || !self.creatures[i].is_alive {
                continue;
            }
            let mut closest_distance = 1000.0;
            let mut closest = None;
            for j in i + 1..self.creatures.len() {
                if i == j {
                    continue;
                };
                if self.creatures[j].last_reproduced < 15 || !self.creatures[j].is_alive {
                    continue;
                }
                let distance = self.creatures[i]
                    .position
                    .distance(&self.creatures[j].position);
                if distance < closest_distance {
                    closest_distance = distance;
                    closest = Some(j);
                }
            }
            if closest_distance > 25.0 {
                continue;
            };
            if let Some(j) = closest {
                new_creatures.push(Creature::from_parents(
                    [&self.creatures[i], &self.creatures[j]],
                ));
                self.creatures[i].last_reproduced = 0;
                self.creatures[j].last_reproduced = 0;
            }
        }
        self.creatures.append(&mut new_creatures);
    }

    pub fn config_as_json(&self) -> String {
        get_config().as_json()
    }

    fn counts(&self) -> ((usize, usize, usize), (usize, usize, usize)) {
        let mut eye_counts = (0, 0, 0);
        let mut skin_counts = (0, 0, 0);
        for creature in &self.creatures {
            match creature.eyes {
                EyeColour::Green => eye_counts.0 += 1,
                EyeColour::Blue => eye_counts.1 += 1,
                EyeColour::Purple => eye_counts.2 += 1,
            }
            match creature.skin {
                SkinColour::Green => skin_counts.0 += 1,
                SkinColour::Yellow => skin_counts.1 += 1,
                SkinColour::Red => skin_counts.2 += 1,
            }
        }
        (eye_counts, skin_counts)
    }

    pub fn info_as_json(&self) -> String {
        let ((green_eyes, blue_eyes, purple_eyes), (green_skin, yellow_skin, red_skin)) =
        self.counts();
        let mut json = String::from("{");
        json.push_str(&format!("\"population\": {},", self.creatures.len()));
        json.push_str(&format!("\"blue_eyes\": {},", blue_eyes));
        json.push_str(&format!("\"green_eyes\": {},", green_eyes));
        json.push_str(&format!("\"purple_eyes\": {},", purple_eyes));
        json.push_str(&format!("\"yellow_skin\": {},", yellow_skin));
        json.push_str(&format!("\"green_skin\": {},", green_skin));
        json.push_str(&format!("\"red_skin\": {},", red_skin));
        json.pop();
        json.push_str("}");
        json
    }
}
