mod vec;

use std::cmp::Ordering;

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

#[macroquad::main("Sim")]
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

    for _ in 0..150 {
        points.push(Point {
            mass: gen_range(1.0, 25.0),
            pos: vec2(gen_range(0.0, screen_width() as f64), gen_range(0.0, screen_height() as f64)),
            vel: vec2(gen_range(-200.0, 200.0), gen_range(-200.0, 200.0)),
            force: vec2(0.0, 0.0)
        });
    }

    loop {
        let dt = get_frame_time() as f64;

        clear_background(WHITE);

        for point in &mut points {
            if is_mouse_button_down(MouseButton::Left) {
                let (mouse_x, mouse_y) = mouse_position();

                point.apply_force(Vec2::from_polar(1000.0 * point.mass, (mouse_y as f64 - point.pos.y).atan2(mouse_x as f64 - point.pos.x)));
            }
/*
            point.apply_force(point.vel * -0.5 * point.mass);

            let contain = vec2(
                screen_width() as f64 - point.pos.x * 2.0,
                screen_height() as f64 - point.pos.y * 2.0
            );

            point.apply_force(Vec2::from_polar(contain.mag().powi(2) / 500.0, contain.angle()));
*/
            point.step(dt);

            //collision with wall
            let coll_xl = point.pos.x - point.mass < 0.0;
            let coll_yt = point.pos.y - point.mass < 0.0;

            let coll_xr = point.pos.x + point.mass > screen_width() as f64;
            let coll_yb = point.pos.y + point.mass > screen_height() as f64;

            let t_x = if coll_xl { point.pos.x - point.mass } else { point.pos.x - screen_width() as f64 + point.mass } / point.vel.x;
            let t_y = if coll_yt { point.pos.y - point.mass } else { point.pos.y - screen_height() as f64 + point.mass } / point.vel.y;

            let coll_x = coll_xl || coll_xr;
            let coll_y = coll_yt || coll_yb;

            let (refl_x, refl_y, t) = match (coll_x, coll_y) {
                (true, true) => {
                    match t_x.total_cmp(&t_y) {
                        Ordering::Less => (false, true, t_y),
                        Ordering::Greater => (true, false, t_x),
                        Ordering::Equal => (true, true, t_x)
                    }
                }
                (true, false) => (true, false, t_x),
                (false, true) => (false, true, t_y),
                (false, false) => (false, false, 0.0)
            };

            point.pos -= point.vel * t;

            if refl_x {
                point.vel.x *= -1.0;
            }
            if refl_y {
                point.vel.y *= -1.0;
            }

            point.pos += point.vel * t;
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
                    let j = ((other.vel - point.vel).dot(n) * 2.0 * point.mass * other.mass) / (point.mass + other.mass);

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