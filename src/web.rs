use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use super::{HEIGHT, WIDTH};
use super::world::World;
use super::config::get_config as get_config_;
use super::config::set_config;

use lazy_static::lazy_static;
use std::sync::Mutex;

const START_TICKS: u32 = 0;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

lazy_static! {
    static ref WORLD: Mutex<World> = Mutex::new(World::new());
}

#[wasm_bindgen]
pub fn initialise(width: f64, height: f64) {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    reinitialise(width, height);
}

#[wasm_bindgen]
pub fn reinitialise(mut width: f64, mut height: f64) {
    let mut world = WORLD.lock().unwrap();
    width /= get_config_().resolution;
    height /= get_config_().resolution;
    set_config("width", format!("{:?}", width).as_str());
    set_config("height", format!("{:?}", height).as_str());
    unsafe {
        WIDTH = get_config_().width;
        HEIGHT = get_config_().height;
    }
    let starting_pop = get_config_().starting_pop;
    *world = World::new();
    world.spawn_random_organisms(starting_pop);
    for _ in 0..START_TICKS {
        world.tick();
    }
}

#[wasm_bindgen]
pub fn render(canvas: HtmlCanvasElement, context: CanvasRenderingContext2d) {
    let world = match WORLD.try_lock() {
        Ok(world) => world,
        Err(_) => return,
    };
    context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
    for creature in &world.creatures {
        creature.render(&context);

    }
}

#[wasm_bindgen]
pub fn render_selected(canvas: HtmlCanvasElement, context: CanvasRenderingContext2d) {
    let mut world = match WORLD.try_lock() {
        Ok(world) => world,
        Err(_) => return,
    };
    context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
    for i in 0..world.creatures.len() {
        if world.creatures[i].selected {
            world.creatures[i].render_selected(&context);
        }
    }
}

#[wasm_bindgen]
pub fn tick() {
    WORLD.lock().unwrap().tick();
}

#[wasm_bindgen]
pub fn update_config(key: &str, value: &str) {
    set_config(key, value)
}

#[wasm_bindgen]
pub fn get_config() -> String {
    get_config_().as_json()
}

#[wasm_bindgen]
pub fn get_world_data() -> String {
    WORLD.lock().unwrap().info_as_json()
}

#[wasm_bindgen]
pub fn on_click(x: f64, y: f64) -> Option<String> {
    let mut closest = 0;
    let mut distance = 1000.0;
    let mut world = WORLD.lock().unwrap();
    for i in 0..world.creatures.len() {
        let creature = &mut world.creatures[i];
        let dx = creature.position.x - x / get_config_().resolution;
        let dy = creature.position.y - y / get_config_().resolution;
        let d = dx * dx + dy * dy;
        if d < distance {
            distance = d;
            closest = i;
        }
        creature.selected = false;
    }
    if distance < 100.0 {
        world.creatures[closest].selected = true;
        let result = 
        return Some(world.creatures[closest].get_gene_info());
    }
    None
}