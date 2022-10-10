use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageBitmap};

use super::config::get_config as get_config_;
use super::config::set_config;
use super::utils::Point;
use super::world::World;
use super::{HEIGHT, WIDTH};

use lazy_static::lazy_static;
use std::sync::Mutex;
// use web_sys::console::log_1;

const START_TICKS: u32 = 0;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

static mut WORLD: Mutex<World> = Mutex::new(World::new());

#[wasm_bindgen]
pub fn initialise(width: f64, height: f64) {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    reinitialise(width, height);
}

#[wasm_bindgen]
pub fn reinitialise(mut width: f64, mut height: f64) {
    width /= get_config_().resolution;
    height /= get_config_().resolution;
    set_config("width", format!("{:?}", width).as_str());
    set_config("height", format!("{:?}", height).as_str());
    unsafe {
        WIDTH = get_config_().width;
        HEIGHT = get_config_().height;
        // log_1(&JsValue::from(format!("{} {}", WIDTH, HEIGHT).as_str()));
        let mut world = WORLD.lock().unwrap();
        *world = World::new();
        for _ in 0..(WIDTH * HEIGHT / 5000.0) as usize {
            world.debris_locs.push(Point::new_random());
        }
        world.spawn_random_organisms(get_config_().starting_pop);
        for _ in 0..get_config_().lifespan * 2 {
            world.tick();
        }
    }
}

#[wasm_bindgen]
pub fn render(canvas: HtmlCanvasElement, context: CanvasRenderingContext2d) {
    context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
    context.set_fill_style(&JsValue::from_str("blue"));
    context.fill_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
    unsafe {
        let mut world = match WORLD.try_lock() {
            Ok(world) => world,
            Err(_) => return,
        };
        world.render(&context);
    }
}

#[wasm_bindgen]
pub fn render_selected(canvas: HtmlCanvasElement, context: CanvasRenderingContext2d) {
    unsafe {
        let mut world = match WORLD.try_lock() {
            Ok(world) => world,
            Err(_) => return,
        };
        context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
        world.render_selected(&context);
    }
}

#[wasm_bindgen]
pub fn tick() {
    unsafe {
        WORLD.lock().unwrap().tick();
    }
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
    unsafe { WORLD.lock().unwrap().info_as_json() }
}

#[wasm_bindgen]
pub fn on_click(x: f64, y: f64) -> Option<String> {
    let mut closest = 0;
    let mut distance = 1000.0;
    unsafe {
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
        }
        if distance < 100.0 {
            world.selected = Some(world.creatures[closest].id);
            Some(world.creatures[closest].get_info_as_json())
        } else {
            world.selected = None;
            None
        }
    }
}

#[wasm_bindgen]
pub fn load_fish(fish: ImageBitmap) {
    unsafe {
        let mut world = WORLD.lock().unwrap();
        world.fish = Some(fish);
    };
}

#[wasm_bindgen]
pub fn load_bg(bg: ImageBitmap) {
    unsafe {
        let mut world = WORLD.lock().unwrap();
        world.bg = Some(bg);
    };
}

#[wasm_bindgen]
pub fn load_debris(debris: ImageBitmap) {
    unsafe {
        let mut world = WORLD.lock().unwrap();
        world.debris = Some(debris);
    };
}
