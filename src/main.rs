mod vec;

use macroquad::{color::colors::*, shapes::*, time::*, window::*};

use vec::{Vec2, vec2};

//TODO
//velocity verlet
//fixed timestep

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
    let mut points = [
        Point {
            mass: 10.0,
            pos: vec2(screen_width() as f64 / 2.0 + 250.0, screen_height() as f64 / 2.0),
            vel: vec2(0.0, 0.0),
            force: vec2(0.0, 0.0)
        },
        Point {
            mass: 100.0,
            pos: vec2(screen_width() as f64 / 2.0, screen_height() as f64 / 2.0),
            vel: vec2(0.0, 0.0),
            force: vec2(0.0, 0.0)
        }
    ];

    let r = points[0].pos.x - screen_width() as f64 / 2.0;

    let distg = ((points[0].pos.x - points[1].pos.x).powi(2) + (points[0].pos.y - points[1].pos.y).powi(2)).sqrt();
    let fg = points[0].mass * points[1].mass / distg.powi(2) * 50000.0;

    let v = (r * fg / points[0].mass).sqrt();

    points[0].vel.y = v;

    loop {
        let dt = get_frame_time() as f64;

        for i in 0..points.len() {
            let (point, others) = points[i..].split_first_mut().unwrap();

            for other in others {
                let dist = ((point.pos.x - other.pos.x).powi(2) + (point.pos.y - other.pos.y).powi(2)).sqrt();
                let force = point.mass * other.mass / dist.powi(2) * 50000.0;

                point.apply_force(Vec2::from_polar(force, (other.pos.y - point.pos.y).atan2(other.pos.x - point.pos.x)));
                //other.apply_force(Vec2::from_polar(force, (point.pos.y - other.pos.y).atan2(point.pos.x - other.pos.x)));
            }
        }

        //ucm
        /*
        let center = vec2(screen_width() as f64 / 2.0, screen_height() as f64 / 2.0);
        let angle = (center - point.pos).angle();
        let force = Vec2::from_polar(fc, angle);

        println!("{}", point.vel.mag());

        point.apply_force(force);
        */
        
        for point in &mut points {
            point.step(dt);
        }

        println!("{}", points[0].vel.mag());

        clear_background(WHITE);

        for point in &points {
            draw_circle(point.pos.x as f32, point.pos.y as f32, point.mass as f32, BLACK);
        }
        draw_circle_lines(screen_width() / 2.0, screen_height() / 2.0, r as f32, 1.0, BLACK);

        next_frame().await
    }
}