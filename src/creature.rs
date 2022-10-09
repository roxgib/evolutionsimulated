use std::f64::consts::{PI, TAU};
use web_sys::CanvasRenderingContext2d;

use crate::config::get_config;

use super::gene::*;
use super::render::*;
use super::utils::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Creature {
    pub position: Point,
    pub direction: Direction,
    pub eye_genes: [EyeColour; 2],
    pub skin_genes: [SkinColour; 2],
    pub speed_genes: [Speed; 2],
    pub eyes: EyeColour,
    pub skin: SkinColour,
    pub speed: Speed,
    pub is_alive: bool,
    pub last_reproduced: u8,
    last_turn: bool,
    hit_wall: u8,
    pub age: u32,
    pub selected: bool,
}

impl Creature {
    fn new(
        position: Point,
        direction: Direction,
        eye_genes: [EyeColour; 2],
        skin_genes: [SkinColour; 2],
        speed_genes: [Speed; 2],
    ) -> Creature {
        Creature {
            position,
            direction,
            eye_genes,
            skin_genes,
            speed_genes,
            eyes: EyeColour::colour(eye_genes[0], eye_genes[1]),
            skin: SkinColour::colour(skin_genes[0], skin_genes[1]),
            speed: Speed::speed(speed_genes[0], speed_genes[1]),
            is_alive: true,
            last_reproduced: 0,
            last_turn: false,
            hit_wall: 0,
            age: 0,
            selected: false,
        }
    }

    pub fn new_random() -> Creature {
        Creature::new(
            Point::new_random(),
            rand::random::<Direction>() % TAU,
            [EyeColour::new_random(), EyeColour::new_random()],
            [SkinColour::new_random(), SkinColour::new_random()],
            [Speed::new_random(), Speed::new_random()],
        )
    }

    pub fn from_parents(parents: [&Creature; 2]) -> Creature {
        Creature::new(
            parents[0].position.midpoint(&parents[1].position),
            rand::random::<Direction>(),
            [
                parents[0].eye_genes[rand::random::<usize>() % 2],
                parents[1].eye_genes[rand::random::<usize>() % 2],
            ],
            [
                parents[0].skin_genes[rand::random::<usize>() % 2],
                parents[1].skin_genes[rand::random::<usize>() % 2],
            ],
            [
                parents[0].speed_genes[rand::random::<usize>() % 2],
                parents[1].speed_genes[rand::random::<usize>() % 2],
            ],
        )
    }

    fn swim(&mut self) {
        let distance = match self.speed {
            Speed::Fast => 2.0,
            Speed::Medium(_) => 1.0,
            Speed::Slow(_) => 0.5,
        };
        self.position.translate3(self.direction, distance);
    }

    pub fn tick(&mut self) {
        self.age += 1;
        if self.age > get_config().lifespan {
            self.is_alive = false;
        }
        if !self.is_alive {
            return;
        }
        self.swim();
        let turn = rand::random::<Direction>() % (PI / 16.0);
        if rand::random::<u8>() < 32 {
            self.last_turn = !self.last_turn;
        }
        if self.last_turn {
            self.direction += turn;
        } else {
            self.direction -= turn;
        }
        self.last_reproduced += 1;
    }

    fn body_positions(&self) -> (Point, Point, Point) {
        (
            self.position.translate4(PI / 2.0 + self.direction, 2.5),
            self.position.translate4(-PI / 2.0 + self.direction, 2.5),
            self.position.translate4(PI + self.direction, 3.0),
        )
    }

    pub fn render(&self, context: &CanvasRenderingContext2d) {
        let eye_colour = match self.eyes {
            EyeColour::Green => "green",
            EyeColour::Blue => "SteelBlue",
            EyeColour::Purple => "purple",
        };
        let skin_colour = match self.skin {
            SkinColour::Red => "DarkRed",
            SkinColour::Green => "DarkGreen",
            SkinColour::Yellow => "Gold",
        };
        let (eye1, eye2, tail) = self.body_positions();
        draw_rectangle(context, skin_colour, tail, 10.0, 3.0, self.direction);
        match self.is_alive {
            true => {
                draw_eye(context, eye_colour, eye1.x, eye1.y, 2.5);
                draw_eye(context, eye_colour, eye2.x, eye2.y, 2.5);
            }
            false => {
                draw_dead_eye(context, eye1, 2.5, self.direction);
                draw_dead_eye(context, eye2, 2.5, self.direction);
            }
        }
        if self.selected {
            draw_outline(context, "red", self.position)
        }
    }

    pub fn render_selected(&mut self, context: &CanvasRenderingContext2d) {
        let position = self.position.clone();
        self.position = Point::new(20.0, 20.0);
        self.render(context);
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

    pub fn get_gene_info(&self) -> String {
        format!(
            "{:?}/{:?}###{:?}/{:?}###{}/{}",
            self.eye_genes[0],
            self.eye_genes[1],
            self.skin_genes[0],
            self.skin_genes[1],
            self.speed_genes[0],
            self.speed_genes[1]
        )
    }
}
