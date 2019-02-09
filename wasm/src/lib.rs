#![allow(unused_variables)]
extern crate cfg_if;
extern crate wasm_bindgen;

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

use std::f64;

use wasm_bindgen::{JsCast, Clamped};


extern crate rand;
use rand::Rng;
use rand::rngs::OsRng;


struct Colors {
    r: u8,
    g: u8,
    b: u8,
}

struct Ball {
    x: u32,
    y: u32,
    vx: i32,
    vy: i32,
    size: u32
}

#[wasm_bindgen(start)]
pub fn start() {
    utils::set_panic_hook();
    let width = 2560;
    let height = 1440;
    let threshold: u32 = 210;
    let colors = Colors {r: 255, g: 0, b: 0};
    
    let num_of_balls = 75;
    let ball_base_size = 50;
    let base_velocity = 1;
    let double_base_velocity = base_velocity * 2;


    // actual canvas
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let mut context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    canvas.set_height(height);
    canvas.set_width(width);

    // virtual canvas
    let temp_canvas = document.create_element("canvas")
        .expect("temp canvas could not be created!")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| println!("temp canvas could not be coerced into html canvas el"))
        .expect("Yea, so temp canvas cannot be created!");

    let mut temp_context = temp_canvas
        .get_context("2d")
        .expect("could not get 2d context")
        .expect("could not get 2d context")
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let mut balls = (0..50).map(|_| {
        // let mut random = OsRng::new().expect("Unable to get new random generator");
        // Ball {
        //     x: random.gen_range(0, width),
        //     y: random.gen_range(0, height),
        //     vx: random.gen_range(0, double_base_velocity) - base_velocity,
        //     vy: random.gen_range(0, double_base_velocity) - base_velocity,
        //     size: random.gen_range(0, ball_base_size) + ball_base_size,
        // }
        Ball {
            x: 100,
            y: 200,
            vx: base_velocity,
            vy: base_velocity,
            size: ball_base_size,
        }
    }).collect::<Vec<Ball>>();

    fn update(
        canvas_width: u32, 
        canvas_height: u32,
        temp_context: &mut web_sys::CanvasRenderingContext2d,
        balls: &mut Vec<Ball>,
        colors: &Colors
    ) {
        temp_context.clear_rect(0.0, 0.0, canvas_width as f64, canvas_height as f64);

        balls
            .iter_mut()
            .for_each(|ball| {
                ball.x = (ball.x as i32 + ball.vx) as u32;
                ball.y = (ball.y as i32 + ball.vy) as u32;

                if ball.x > canvas_width + ball.size {
                    ball.x = 0;
                } else if ball.x == 0 {
                    ball.x = canvas_width + ball.size;
                }

                if ball.y > canvas_height + ball.size {
                    ball.y = 0;
                } else if ball.y == 0 {
                    ball.y = canvas_height + ball.size;
                }
                temp_context.begin_path();
                let grad = temp_context
                    .create_radial_gradient(ball.x as f64, ball.y as f64, 1.0, ball.x as f64, ball.y as f64, ball.size as f64)
                    .expect("Unable to create Radial Pattern");

                grad.add_color_stop(0.0, &format!("rgba({}, {}, {}, 1)", colors.r, colors.g, colors.b)).expect("Unable to add color stop to alpah 1");
                grad.add_color_stop(0.0, &format!("rgba({}, {}, {}, 0)", colors.r, colors.g, colors.b)).expect("Unable to add color stop to alpah 0");;
                temp_context.set_fill_style(&grad);
                temp_context.arc(ball.x as f64, ball.y as f64, ball.size as f64, 0.0, f64::consts::PI * 2.0).expect("Unable to arc");
                temp_context.fill();
                temp_context.set_line_width(2.0);
            });
    }

    update(width, height, &mut temp_context, &mut balls, &colors);

    fn metabalize(
        width: f64,
        height: f64,
        temp_context: &web_sys::CanvasRenderingContext2d,
        context: &mut web_sys::CanvasRenderingContext2d,
        threshold: u8
    ) {
        let mut image_data = temp_context.get_image_data(0.0, 0.0, width, height).expect("Unable to get image data");
        let mut pix = image_data.data();

        (0..pix.len()).enumerate().step_by(4).for_each(|(i, _)| {
            let mut current_pix_alpha = pix[i + 3];

            if current_pix_alpha < threshold {
                current_pix_alpha = (0.167 * current_pix_alpha as f64) as u8;
                if current_pix_alpha > (threshold as f64 * 0.25) as u8 {
                    current_pix_alpha = 0;
                }
            }

            pix[i + 3] = current_pix_alpha;
        });
        let new_image_data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut pix), width as u32, height as u32).expect("cannot generate image data");
         
        temp_context.put_image_data(&new_image_data, 0.0, 0.0);
    }

    metabalize(width as f64, height as f64, &temp_context, &mut context, threshold as u8)



    // context.begin_path();

    // // Draw the outer circle.
    // context
    //     .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
    //     .unwrap();

    // // Draw the mouth.
    // context.move_to(110.0, 75.0);
    // context.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();

    // // Draw the left eye.
    // context.move_to(65.0, 65.0);
    // context
    //     .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
    //     .unwrap();

    // // Draw the right eye.
    // context.move_to(95.0, 65.0);
    // context
    //     .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
    //     .unwrap();

    // context.stroke();
}
