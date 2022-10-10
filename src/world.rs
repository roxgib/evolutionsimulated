use rand::seq::SliceRandom;

use crate::render::{draw_debris, draw_fish, render_bg};
use crate::utils::Point;
use crate::{config::get_config, render::draw_outline};

use super::creature::Creature;
use super::gene::ColourGene;
use web_sys::{CanvasRenderingContext2d, ImageBitmap};

pub struct World {
    pub creatures: Vec<Creature>,
    pub fish: Option<ImageBitmap>,
    pub bg: Option<ImageBitmap>,
    pub debris: Option<ImageBitmap>,
    pub debris_locs: Vec<Point>,
    pub frame_counter: u8,
    pub selected: Option<u32>,
    direction: f64,
    focus: Point,
}

impl World {
    pub const fn new() -> World {
        World {
            creatures: Vec::new(),
            fish: None,
            bg: None,
            debris: None,
            debris_locs: Vec::new(),
            frame_counter: 0,
            selected: None,
            direction: 0.0,
            focus: Point { x: 0.0, y: 0.0 },
        }
    }

    pub fn spawn_random_organisms(&mut self, count: usize) {
        for _ in 0..count {
            self.creatures.push(Creature::new_random());
        }
    }

    pub fn tick(&mut self) {
        self.direction += rand::random::<f64>();
        self.direction %= std::f64::consts::TAU;
        if rand::random::<u8>() < 8 {
            self.focus = Point::new_random();
        }
        for creature in &mut self.creatures {
            let direction = creature.position.direction_to(&self.focus);
            creature.tick(direction);
        }
        self.creatures
            .retain(|creature| creature.age < get_config().lifespan + 15);
        if self.creatures.len() < get_config().max_creatures {
            self.reproduce();
        }
    }

    fn reproduce(&mut self) {
        let mut new_creatures = Vec::new();
        self.creatures.shuffle(&mut rand::thread_rng());
        for i in 0..self.creatures.len() {
            if new_creatures.len() >= self.creatures.len() / 20 {
                break;
            }
            {
                let creature = &self.creatures[i];
                if creature.last_reproduced < 30 || !creature.is_alive || creature.age < 50 {
                    continue;
                }
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
                let new_creature = Creature::from_parents([&self.creatures[i], &self.creatures[j]]);
                self.creatures[i].last_reproduced = 0;
                self.creatures[j].last_reproduced = 0;
                self.creatures[i].offspring.push(new_creature.id);
                self.creatures[j].offspring.push(new_creature.id);
                new_creatures.push(new_creature);
            }
        }
        self.creatures.append(&mut new_creatures);
    }

    pub fn config_as_json(&self) -> String {
        get_config().as_json()
    }

    fn counts(&self) -> (u16, u16, u16, u16, u16, u16, u16) {
        let mut colour_counts = (0, 0, 0, 0, 0, 0, 0);
        for creature in &self.creatures {
            match creature.colour {
                ColourGene::Orange => colour_counts.0 += 1,
                ColourGene::Red => colour_counts.1 += 1,
                ColourGene::LBlue => colour_counts.2 += 1,
                ColourGene::DBlue => colour_counts.3 += 1,
                ColourGene::Black => colour_counts.4 += 1,
                ColourGene::Yellow => colour_counts.5 += 1,
                ColourGene::Purple => colour_counts.6 += 1,
            }
        }
        colour_counts
    }

    pub fn info_as_json(&self) -> String {
        let (
            orange_skin,
            red_skin,
            light_blue_skin,
            dark_blue_skin,
            black_skin,
            yellow_skin,
            purple_skin,
        ) = self.counts();
        let mut json = String::from("{");
        json.push_str(&format!("\"population\": {},", self.creatures.len()));
        json.push_str(&format!("\"orange_skin\": {},", orange_skin));
        json.push_str(&format!("\"red_skin\": {},", red_skin));
        json.push_str(&format!("\"light_blue_skin\": {},", light_blue_skin));
        json.push_str(&format!("\"dark_blue_skin\": {},", dark_blue_skin));
        json.push_str(&format!("\"black_skin\": {},", black_skin));
        json.push_str(&format!("\"yellow_skin\": {},", yellow_skin));
        json.push_str(&format!("\"purple_skin\": {}", purple_skin));

        if let Some(id) = self.selected {
            for creature in &self.creatures {
                if creature.id == id {
                    json.push_str(",");
                    json.push_str("\"selected\": ");
                    json.push_str(&creature.get_info_as_json());
                }
            }
        }

        json.push_str("}");
        json
    }

    pub fn render(&mut self, context: &CanvasRenderingContext2d) {
        if let Some(bg) = &self.bg {
            render_bg(context, bg);
        }
        if let Some(debris) = &self.debris {
            for loc in &self.debris_locs {
                draw_debris(context, &debris, *loc)
            }
        }
        if let Some(image) = &self.fish {
            let selected_id = self.selected.unwrap_or(0);
            for creature in &self.creatures {
                let colour = creature.colour as u8;
                let frame_counter = if creature.is_alive {
                    (self.frame_counter / 4) % 8
                } else {
                    0
                };
                if creature.id == selected_id {
                    draw_outline(context, "red", creature.position)
                } else if creature.parents.contains(&selected_id) {
                    draw_outline(context, "blue", creature.position)
                } else if creature.offspring.contains(&selected_id) {
                    draw_outline(context, "green", creature.position)
                }
                draw_fish(
                    context,
                    colour,
                    frame_counter,
                    creature.position,
                    creature.direction,
                    image,
                    creature.age,
                    false,
                )
            }
        }
        self.frame_counter = self.frame_counter.wrapping_add(1);
    }

    pub fn render_selected(&mut self, context: &CanvasRenderingContext2d) {
        if let (Some(id), Some(image)) = (self.selected, &self.fish) {
            for creature in &self.creatures {
                if creature.id != id {
                    continue;
                }
                let colour = creature.colour as u8;
                let frame_counter = if creature.is_alive {
                    (self.frame_counter / 4) % 8
                } else {
                    0
                };
                draw_fish(
                    context,
                    colour,
                    frame_counter,
                    crate::utils::Point { x: 25.0, y: 25.0 },
                    creature.direction,
                    image,
                    creature.age,
                    true,
                )
            }
        }
        self.frame_counter = self.frame_counter.wrapping_add(1);
    }
}
