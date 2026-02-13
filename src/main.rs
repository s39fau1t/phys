mod vec;

use macroquad::{color::colors::*, input::*, rand::*, shapes::*, time::*, window::*};

use vec::{Vec2, vec2};

#[derive(Debug)]
struct Point {
    mass: f64,
    pos: Vec2,
    vel: Vec2,
    force: Vec2
}

impl Point {
    fn apply_force(&mut self, force: Vec2) {
        self.force = self.force + force;
    }

    fn step(&mut self, dt: f64) {
        self.vel += self.force / self.mass * dt;

        self.pos += self.vel * dt;

        self.force = Vec2 { x: 0.0, y: 0.0 }
    }
}

#[macroquad::main("MyGame")]
async fn main() {
    /*
    let mut points = [
        Point {
            mass: 5.0,
            pos: Vec2 { x: screen_width() as f64 / 4.0 , y: screen_height() as f64 / 2.0 },
            vel: Vec2 { x: 50.0, y: -50.0 },
            force: Vec2 { x: 0.0, y: 0.0 }
        },
        Point {
            mass: 5.0,
            pos: Vec2 { x: screen_width() as f64 / 4.0 * 3.0 , y: screen_height() as f64 / 2.0 },
            vel: Vec2 { x: -50.0, y: 50.0 },
            force: Vec2 { x: 0.0, y: 0.0 }
        },
        Point {
            mass: 20.0,
            pos: Vec2 { x: screen_width() as f64 / 2.0 , y: screen_height() as f64 / 2.0 },
            vel: Vec2 { x: 0.0, y: 0.0 },
            force: Vec2 { x: 0.0, y: 0.0 }
        }
    ];
    */

    let mut points = Vec::new();

    for _ in 0..100 {
        points.push(Point {
            mass: gen_range(5.0, 25.0),
            pos: vec2(gen_range(0.0, screen_width() as f64), gen_range(0.0, screen_height() as f64)),
            vel: vec2(gen_range(-200.0, 200.0), gen_range(-200.0, 200.0)),
            force: vec2(0.0, 0.0)
        })
    }

    loop {
        clear_background(WHITE);

        for i in 0..points.len() {
            let (point, others) = points[i..].split_first_mut().unwrap();

            for other in others {
                let dist = ((point.pos.x - other.pos.x).powi(2) + (point.pos.y - other.pos.y).powi(2)).sqrt();
                let force = point.mass * other.mass * dist / 1000.0;
                //let force = point.mass * other.mass / dist.powi(2) * 50000.0;

                point.apply_force(Vec2::from_polar(force, (other.pos.y - point.pos.y).atan2(other.pos.x - point.pos.x)));
                other.apply_force(Vec2::from_polar(force, (point.pos.y - other.pos.y).atan2(point.pos.x - other.pos.x)));
            }
        }

        for point in &mut points {
            if is_mouse_button_down(MouseButton::Left) {
                let (mouse_x, mouse_y) = mouse_position();

                point.apply_force(Vec2::from_polar(1000.0 * point.mass, (mouse_y as f64 - point.pos.y).atan2(mouse_x as f64 - point.pos.x)));
            }

            let contain = vec2(
                screen_width() as f64 - point.pos.x * 2.0,
                screen_height() as f64 - point.pos.y * 2.0
            );

            point.apply_force(Vec2::from_polar(contain.mag().powi(2) / 500.0, contain.angle()));

            point.step(get_frame_time() as f64);

            draw_circle(point.pos.x as f32, point.pos.y as f32, point.mass as f32, BLACK);
        }

        next_frame().await
    }
}