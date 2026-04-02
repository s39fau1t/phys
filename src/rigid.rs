use crate::vec::{Vec2, vec2};

pub struct RigidBody {
    pub mass: f64,
    pub width: f64,
    pub height: f64,
    //center of body
    pub pos: Vec2,
    pub vel: Vec2,
    pub force: Vec2,
    pub angle: f64,
    pub ang_vel: f64,
    pub torque: f64,
}

impl RigidBody {
    pub fn apply_force(&mut self, fpos: Vec2, force: Vec2) {
        //TODO check if fpos is on body
        self.force += force;

        let r = fpos - self.pos;
        self.torque += r.cross(force);
    }

    //TODO: convert to verlet instead
    pub fn step(&mut self, dt: f64) {
        self.vel += self.force / self.mass * dt;
        self.pos += self.vel * dt;

        let mi = self.mass * (self.width.powi(2) * self.height.powi(2)) / 12.0;
        self.ang_vel += self.torque / mi * dt;
        self.angle += self.ang_vel * dt;

        self.force = vec2(0.0, 0.0);
        self.torque = 0.0;
    }
}
