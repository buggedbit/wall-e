use ggez::nalgebra::Point2;
use ggez::*;
use rand::Rng;
use trail::Trail;

mod trail {
    use super::*;

    pub struct Trail {
        queue: Vec<Point2<f32>>,
        limit: usize,
    }

    impl Trail {
        pub fn new(limit: usize) -> Trail {
            Trail {
                queue: Vec::with_capacity(limit),
                limit: limit,
            }
        }

        pub fn add(&mut self, x: f32, y: f32) {
            if self.queue.len() > 0 {
                let new_point = Point2::new(x, y);
                if self.queue[self.queue.len() - 1] != new_point {
                    self.queue.push(Point2::new(x, y));
                }
            } else {
                self.queue.push(Point2::new(x, y));
            }
            if self.queue.len() > self.limit {
                self.queue.remove(0);
            }
        }

        pub fn draw(&self, ctx: &mut ggez::Context) -> ggez::GameResult {
            if self.queue.len() > 1 {
                let line = graphics::Mesh::new_line(
                    ctx,
                    &self.queue,
                    2.0,
                    graphics::Color::from((0.0, 1.0, 1.0)),
                )?;
                graphics::draw(ctx, &line, (Point2::new(0.0, 0.0),))?;
            }
            Ok(())
        }
    }
}

fn clamp(v: f32, min_max: (f32, f32)) -> f32 {
    let (min, max) = min_max;
    if v < min {
        return min;
    }
    if v > max {
        return max;
    }
    v
}

pub struct DiffDriveModel {
    x: f32,
    y: f32,
    or_in_rad: f32,
    radius: f32,
    v: f32,
    w: f32,
    trail: Trail,
    scale: f32,
    goal: (f32, f32),
}

impl DiffDriveModel {
    const V_BOUNDS: (f32, f32) = (0.0, 20.0);
    const W_BOUNDS: (f32, f32) = (-1.0, 1.0);
    const TRIAL_LENGTH: usize = 500;

    pub fn spawn_randomly(
        x_bounds: (f32, f32),
        y_bounds: (f32, f32),
        or_bounds: (f32, f32),
        radius: f32,
        goal: (f32, f32),
    ) -> DiffDriveModel {
        // Compile time asserts
        const_assert!(DiffDriveModel::V_BOUNDS.0 < DiffDriveModel::V_BOUNDS.1);
        const_assert!(DiffDriveModel::W_BOUNDS.0 < DiffDriveModel::W_BOUNDS.1);
        // Spawn at random location
        let mut rng = rand::thread_rng();
        let x = x_bounds.0 + (x_bounds.1 - x_bounds.0) * rng.gen::<f32>();
        let y = y_bounds.0 + (y_bounds.1 - y_bounds.0) * rng.gen::<f32>();
        let or = or_bounds.0 + (or_bounds.1 - or_bounds.0) * rng.gen::<f32>();
        // Trail config
        let mut trail = Trail::new(DiffDriveModel::TRIAL_LENGTH);
        trail.add(x, y);
        // Normalized scale w.r.t goal
        let scale = {
            let scale_x = (goal.0 - x).abs();
            let scale_y = (goal.1 - y).abs();
            (scale_x * scale_x + scale_y * scale_y).sqrt()
        };
        DiffDriveModel {
            x: x,
            y: y,
            or_in_rad: or,
            radius: radius,
            v: 0.0,
            w: 0.0,
            trail: trail,
            scale: scale,
            goal: goal,
        }
    }

    pub fn update(&mut self, dt: f32) -> ggez::GameResult {
        self.x += self.v * self.or_in_rad.cos() * dt;
        self.y += self.v * self.or_in_rad.sin() * dt;
        self.or_in_rad += self.w * dt;
        self.trail.add(self.x, self.y);
        Ok(())
    }

    pub fn draw(&self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Point2::new(self.x, self.y),
            self.radius,
            0.5,
            graphics::WHITE,
        )?;
        graphics::draw(ctx, &circle, (Point2::new(0.0, 0.0),))?;

        let line = graphics::Mesh::new_line(
            ctx,
            &[
                Point2::new(self.x, self.y),
                Point2::new(
                    self.x + self.radius * self.or_in_rad.cos(),
                    self.y + self.radius * self.or_in_rad.sin(),
                ),
            ],
            2.0,
            graphics::Color::from((1.0, 0.0, 0.0)),
        )?;
        graphics::draw(ctx, &line, (Point2::new(0.0, 0.0),))?;

        self.trail.draw(ctx)?;
        Ok(())
    }

    pub fn control(&self) -> (f32, f32) {
        (self.v, self.w)
    }

    // pub fn state(&self) -> (f32, f32, f32) {
    //     (self.x, self.y, self.or_in_rad)
    // }

    pub fn scaled_state(&self) -> (f32, f32, f32) {
        (
            (self.goal.0 - self.x) / self.scale,
            (self.goal.1 - self.y) / self.scale,
            self.or_in_rad % std::f32::consts::PI,
        )
    }

    pub fn increment_control(&mut self, dv: f32, dw: f32) {
        self.v = clamp(self.v + dv, DiffDriveModel::V_BOUNDS);
        self.w = clamp(self.w + dw, DiffDriveModel::W_BOUNDS);
    }

    pub fn set_control(&mut self, v: f32, w: f32) {
        self.v = clamp(v, DiffDriveModel::V_BOUNDS);
        self.w = clamp(w, DiffDriveModel::W_BOUNDS);
    }
}
