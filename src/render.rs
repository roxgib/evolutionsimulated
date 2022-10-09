use super::config::get_config;
use super::utils::Point;
use std::f64::consts::PI;
use wasm_bindgen::prelude::JsValue;
use web_sys::CanvasRenderingContext2d;

pub fn draw_rectangle(
    context: &CanvasRenderingContext2d,
    colour: &str,
    loc: Point,
    w: f64,
    h: f64,
    rot: f64,
) {
    let res = get_config().resolution;
    context.set_fill_style(&JsValue::from_str(colour));
    context.translate(loc.x * res, loc.y * res).unwrap();
    context.rotate(rot).unwrap();
    context.fill_rect(-w / 2.0 * res, -h / 2.0 * res, w * res, h * res);
    context.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0).unwrap();
}

pub fn draw_circle(context: &CanvasRenderingContext2d, colour: &str, x: f64, y: f64, r: f64) {
    let res = get_config().resolution;
    context.set_fill_style(&JsValue::from_str(colour));
    context.begin_path();
    context
        .arc(x * res, y * res, r * res, 0.0, std::f64::consts::TAU)
        .unwrap();
    context.fill();
    context.stroke();
}

pub fn draw_eye(context: &CanvasRenderingContext2d, colour: &str, x: f64, y: f64, r: f64) {
    draw_circle(context, "white", x, y, r);
    draw_circle(context, colour, x, y, r / 1.3);
    draw_circle(context, "black", x, y, r / 4.0);
}

pub fn draw_dead_eye(context: &CanvasRenderingContext2d, loc: Point, r: f64, d: f64) {
    draw_circle(context, "white", loc.x, loc.y, r);
    draw_circle(context, "grey", loc.x, loc.y, r / 1.3);
    draw_rectangle(context, "black", loc, r / 1.2, r / 4.0, d - PI / 4.0);
    draw_rectangle(context, "black", loc, r / 1.2, r / 4.0, d + PI / 4.0);
}
pub fn draw_outline(context: &CanvasRenderingContext2d, colour: &str, loc: Point) {
    let res = get_config().resolution;
    context.set_stroke_style(&JsValue::from_str(colour));
    context.stroke_rect((loc.x - 10.0) * res, (loc.y - 10.0) * res, 20.0 * res, 20.0 * res);
    context.set_stroke_style(&JsValue::from_str("black"));
}