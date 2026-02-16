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
            mass: 15.0,
            pos: Vec2 { x: screen_width() as f64 / 4.0 , y: screen_height() as f64 / 2.0 },
            vel: Vec2 { x: 100.0, y: 0.0 },
            force: Vec2 { x: 0.0, y: 0.0 }
        },
        Point {
            mass: 15.0,
            pos: Vec2 { x: screen_width() as f64 / 4.0 * 3.0 , y: screen_height() as f64 / 2.0 },
            vel: Vec2 { x: -100.0, y: 0.0 },
            force: Vec2 { x: 0.0, y: 0.0 }
        }
    ];
    */

    let mut points = Vec::new();

    for _ in 0..100 {
        points.push(Point {
            mass: gen_range(1.0, 25.0),
            pos: vec2(gen_range(0.0, screen_width() as f64), gen_range(0.0, screen_height() as f64)),
            vel: vec2(gen_range(-200.0, 200.0), gen_range(-200.0, 200.0)),
            force: vec2(0.0, 0.0)
        });
    }

    loop {
        let dt = get_frame_time() as f64 / 5.0;

        clear_background(WHITE);

        for point in &mut points {
            point.step(dt);
        }

        for i in 0..points.len() {
            let (point, others) = points[i..].split_first_mut().unwrap();

            for other in others {
                //collision (todo: sat)
                //right now radius is based on mass
                if point.pos.dist(other.pos) < point.mass + other.mass {
                    //assume constant velocity
                    //TODO: factor in acceleration a*t^2
                    //solve the quadratic dist(p1 - v1*t, p2 - v2*t)=r1+r2
                    let a = point.vel.mag_squared() + other.vel.mag_squared() + point.vel.dot(other.vel);
                    let b = 2.0 * (point.vel - other.vel).dot(other.pos - point.pos);
                    let c = (point.pos - other.pos).mag_squared() - (point.mass + other.mass).powi(2);

                    let t = (-b + (b.powi(2) - 4.0 * a * c).sqrt()) / (2.0 * a);

                    //resolve collsion
                    point.pos -= point.vel * t;
                    other.pos -= other.vel * t;

                    //use conservation of momentum to find new velocities (assume perfectly elastic)
                    //direction of impulse
                    let n = (point.pos - other.pos).norm();
                    //magnitude of impulse
                    let j = (((point.vel - other.vel) * 2.0 * point.mass * other.mass) / (point.mass + other.mass)).mag();

                    point.vel += n * j / point.mass;
                    other.vel -= n * j / other.mass;

                    point.pos += point.vel * t;
                    other.pos += other.vel * t;
                }
            }
        }

        for point in &points {
            draw_circle(point.pos.x as f32, point.pos.y as f32, point.mass as f32, BLACK);
        }

        next_frame().await
    }
}