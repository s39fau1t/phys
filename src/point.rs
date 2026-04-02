const G: f64 = 1_000_000.0;

#[derive(Debug)]
struct Point {
    mass: f64,
    pos: Vec2,
    vel: Vec2,
    force: Vec2,
}

impl Point {
    fn apply_force(&mut self, force: Vec2) {
        self.force += force;
    }
    /*
        fn step(&mut self, dt: f64) {
            self.vel += self.force / self.mass * dt;

            self.pos += self.vel * dt;

            self.force = Vec2 { x: 0.0, y: 0.0 }
        }
    */
}

fn apply_forces(points: &mut [Point]) {
    for i in 0..points.len() {
        let (point, others) = points[i..].split_first_mut().unwrap();

        for other in others {
            let dist =
                ((point.pos.x - other.pos.x).powi(2) + (point.pos.y - other.pos.y).powi(2)).sqrt();
            let force = point.mass * other.mass / dist.powi(2) * G;

            point.apply_force(Vec2::from_polar(
                force,
                (other.pos.y - point.pos.y).atan2(other.pos.x - point.pos.x),
            ));
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
}

fn step(points: &mut [Point], dt: f64) {
    for point in &mut points[..] {
        point.vel += point.force / point.mass * 0.5 * dt;
        point.pos += point.vel * dt;

        point.force = vec2(0.0, 0.0);
    }

    apply_forces(points);

    for point in &mut points[..] {
        point.vel += point.force / point.mass * 0.5 * dt;
    }
}
