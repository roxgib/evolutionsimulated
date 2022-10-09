pub static mut CONFIG: Config = Config::new();

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub width: f64,
    pub height: f64,
    pub max_creatures: usize,
    pub starting_pop: usize,
    pub resolution: f64,
    pub lifespan: u32,
}

impl Config {
    pub const fn new() -> Config {
        Config {
            // World Options
            width: 250.0,
            height: 250.0,
            resolution: 4.0,
            starting_pop: 20,
            max_creatures: 100,
            lifespan: 150,
        }
    }

    pub fn as_json(&self) -> String {
        String::from(
            r#"{
            "width": "#,
        ) + &self.width.to_string()
            + r#",
            "height": "#
            + &self.height.to_string()
            + r#",
            "resolution": "#
            + &self.resolution.to_string()
            + r#",
            "starting_pop": "#
            + &self.starting_pop.to_string()
            + r#",
            "max_creatures": "#
            + &self.max_creatures.to_string()
            + r#",
            "lifespan": "#
            + &self.lifespan.to_string()
            + r#"
        }"#
    }
}

pub fn get_config() -> Config {
    unsafe { CONFIG.clone() }
}

pub fn set_config(key: &str, value: &str) {
    unsafe {
        match key {
            "starting_pop" => CONFIG.starting_pop = value.parse::<usize>().unwrap(),
            "width" => CONFIG.width = value.parse::<f64>().unwrap(),
            "height" => CONFIG.height = value.parse::<f64>().unwrap(),
            "max_creatures" => CONFIG.max_creatures = value.parse::<usize>().unwrap(),
            "resolution" => CONFIG.resolution = value.parse::<f64>().unwrap(),
            _ => {}
        }
    }
}
