use std::f64::consts::{PI, TAU};
use web_sys::CanvasRenderingContext2d;

use crate::config::get_config;

use super::gene::*;
use super::utils::*;

static mut ID: u32 = 0;

fn get_id() -> u32 {
    unsafe {
        ID += 1;
        ID
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Creature {
    pub id: u32,
    pub position: Point,
    pub direction: Direction,
    pub colour_genes: [ColourGene; 2],
    pub speed_genes: [Speed; 2],
    pub colour: ColourGene,
    pub speed: Speed,
    pub is_alive: bool,
    pub last_reproduced: u8,
    pub parents: [u32; 2],
    pub offspring: Vec<u32>,
    last_turn: bool,
    hit_wall: u8,
    pub age: u32,
}

impl Creature {
    fn new(
        position: Point,
        direction: Direction,
        colour_genes: [ColourGene; 2],
        speed_genes: [Speed; 2],
        parents: Option<[u32; 2]>,
    ) -> Creature {
        Creature {
            id: get_id(),
            position,
            direction,
            colour_genes,
            speed_genes,
            colour: ColourGene::colour(colour_genes[0], colour_genes[1]),
            speed: Speed::speed(speed_genes[0], speed_genes[1]),
            is_alive: true,
            last_reproduced: 0,
            parents: parents.unwrap_or([0, 0]),
            offspring: Vec::new(),
            last_turn: false,
            hit_wall: 0,
            age: 0,
        }
    }

    pub fn new_random() -> Creature {
        Creature::new(
            Point::new_random(),
            rand::random::<Direction>() % TAU,
            [ColourGene::new_random(), ColourGene::new_random()],
            [Speed::new_random(), Speed::new_random()],
            None,
        )
    }

    pub fn from_parents(parents: [&Creature; 2]) -> Creature {
        Creature::new(
            parents[0].position.midpoint(&parents[1].position),
            rand::random::<Direction>(),
            [
                parents[0].colour_genes[rand::random::<usize>() % 2],
                parents[1].colour_genes[rand::random::<usize>() % 2],
            ],
            [
                parents[0].speed_genes[rand::random::<usize>() % 2],
                parents[1].speed_genes[rand::random::<usize>() % 2],
            ],
            Some([parents[0].id, parents[1].id]),
        )
    }

    fn swim(&mut self) {
        let distance = match self.speed {
            Speed::Fast => 2.5,
            Speed::Medium(_) => 2.0,
            Speed::Slow(_) => 1.5,
        };
        let distance = self.age.min(30) as f64 * distance / 30.0;
        self.position.translate3(self.direction, distance);
    }

    pub fn tick(&mut self, direction: Direction) {
        self.age += 1;
        if self.age > get_config().lifespan {
            self.is_alive = false;
        }
        if !self.is_alive {
            return;
        }
        self.last_reproduced += 1;
        
        self.swim();
        let turn = rand::random::<Direction>() % (PI / 32.0);
        let r = rand::random::<u8>();
        if ((self.direction < direction) != self.last_turn && r < 32) || r < 4 {
            self.last_turn = !self.last_turn;
        }
        if self.last_turn {
            self.direction += turn;
        } else {
            self.direction -= turn;
        }
        self.direction %= TAU;
    }

    fn body_positions(&self) -> (Point, Point, Point) {
        (
            self.position.translate4(PI / 2.0 + self.direction, 2.5),
            self.position.translate4(-PI / 2.0 + self.direction, 2.5),
            self.position.translate4(PI + self.direction, 3.0),
        )
    }

    pub fn render_selected(&mut self, context: &CanvasRenderingContext2d) {
        let position = self.position.clone();
        self.position = Point::new(20.0, 20.0);
        // self.render(context);
        self.position = position;
    }

    fn does_overlap(creature1: &Creature, creature2: &Creature) -> bool {
        if creature1.position.distance(&creature2.position) > 10.0 {
            return false;
        }
        let (eye1, eye2, tail) = creature1.body_positions();
        let (eye3, eye4, tail2) = creature2.body_positions();
        if eye1.distance(&eye3) < 10.0
            || eye1.distance(&eye4) < 10.0
            || eye2.distance(&eye3) < 10.0
            || eye2.distance(&eye4) < 10.0
        {
            return true;
        };
        if tail.distance(&eye3) < 10.0
            || tail.distance(&eye4) < 10.0
            || tail2.distance(&eye1) < 10.0
            || tail2.distance(&eye2) < 10.0
            || tail.distance(&tail2) < 10.0
        {
            return true;
        };
        false
    }

    pub fn get_info_as_json(&self) -> String {
        format!("{{\"age\": {}, \"speed\": \"{}\", \"speed_genes\": \"{}/{}\", \"colour_genes\": \"{}/{}\", \"colour\": \"{:?}\", \"offspring\": {}, \"last_reproduced\": {}}}", self.age, self.speed, self.speed_genes[0], self.speed_genes[1], self.colour_genes[0], self.colour_genes[1], self.colour, self.offspring.len(), self.last_reproduced)
    }
}
