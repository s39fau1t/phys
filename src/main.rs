mod rigid;
mod vec;

use macroquad::prelude::*;

use rigid::RigidBody;
use vec::{Vec2, vec2};

const TIME_STEP: f64 = 1.0 / 240.0;

#[macroquad::main("Sim")]
async fn main() {
    let mut rigid_body = RigidBody {
        mass: 1.0,
        width: 50.0,
        height: 50.0,
        //center of body
        pos: vec2(screen_width() as f64 / 2.0, screen_height() as f64 / 2.0),
        vel: vec2(0.0, 0.0),
        force: vec2(0.0, 0.0),
        angle: 0.0,
        ang_vel: 0.0,
        torque: 0.0,
    };

    let mut acct = 0.0;

    loop {
        acct += get_frame_time() as f64;

        while acct >= TIME_STEP {
            rigid_body.apply_force(rigid_body.pos + vec2(rigid_body.width / 2.0, rigid_body.height / 2.0), vec2(0.0, -10000.0));
            rigid_body.apply_force(rigid_body.pos + vec2(-rigid_body.width / 2.0, -rigid_body.height / 2.0), vec2(0.0, 10000.0));
            //step
            rigid_body.step(TIME_STEP);

            acct -= TIME_STEP;
        }

        clear_background(WHITE);

        //draw
        draw_rectangle_lines_ex(
            rigid_body.pos.x as f32,
            rigid_body.pos.y as f32,
            rigid_body.width as f32,
            rigid_body.height as f32,
            1.0,
            DrawRectangleParams {
                offset: macroquad::math::vec2(0.5, 0.5),
                rotation: rigid_body.angle as f32,
                color: BLACK,
            },
        );

        next_frame().await
    }
}
