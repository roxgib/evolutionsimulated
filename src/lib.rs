mod utils;
mod gene;
mod config;
pub mod world;
mod creature;
mod render;

#[cfg(target_arch = "wasm32")]
pub mod web;

pub static mut HEIGHT: f64 = 0.0;
pub static mut WIDTH: f64 = 0.0;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
