use super::config::get_config;
use super::utils::Point;
use std::f64::consts::PI;
use wasm_bindgen::prelude::JsValue;
use web_sys::CanvasRenderingContext2d;
use web_sys::ImageBitmap;

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
    context.stroke_rect(
        (loc.x - 10.0) * res,
        (loc.y - 10.0) * res,
        20.0 * res,
        20.0 * res,
    );
    context.set_stroke_style(&JsValue::from_str("black"));
}

pub fn render_bg(context: &CanvasRenderingContext2d, bg: &ImageBitmap) {
    let res = get_config().resolution;
    let canvas = context.canvas().unwrap();
    let scale_factor = 256 * res as u32;
    let sace_factor_f = scale_factor as f64;
    for i in 0..canvas.width() / scale_factor + 1 {
        for j in 0..canvas.height() / scale_factor + 1 {
            context
                .draw_image_with_image_bitmap_and_dw_and_dh(
                    bg,
                    i as f64 * sace_factor_f,
                    j as f64 * sace_factor_f,
                    sace_factor_f,
                    sace_factor_f,
                )
                .unwrap();
        }
    }
}

pub fn draw_fish(
    context: &CanvasRenderingContext2d,
    species: u8,
    frame_counter: u8,
    loc: Point,
    rot: f64,
    image: &ImageBitmap,
    size: u32,
    big: bool,
) {
    let res = get_config().resolution;
    let mut size = res / 100.0 * size.min(50) as f64;
    if big {
        size *= 1.5;
    }
    context.translate(loc.x * res, loc.y * res).unwrap();
    context.rotate(rot).unwrap();
    context
        .draw_image_with_image_bitmap_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            image,
            (species * 37) as f64,
            (frame_counter * 21) as f64,
            37.0,
            21.0,
            -37.0 * size / 2.0,
            -21.0 * size / 2.0,
            37.0 * size,
            21.0 * size,
        )
        .unwrap();
    context.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0).unwrap();
}

pub fn draw_debris(context: &CanvasRenderingContext2d, image: &ImageBitmap, loc: Point) {
    let res = get_config().resolution;
    context.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0).unwrap();
    context.translate(loc.x * res, loc.y * res).unwrap();
    context.rotate(loc.y * 10.0).unwrap();
    context
        .draw_image_with_image_bitmap_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            image,
            ((loc.x / 0.8 + loc.y * 1.75) as u32 % 27 * 32) as f64,
            0.0,
            32.0,
            64.0,
            0.0,
            0.0,
            12.0 * res,
            24.0 * res,
        )
        .unwrap();
    context.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0).unwrap();
}
