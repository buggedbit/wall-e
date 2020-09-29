use super::Experiment;
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::*;
use ndarray::prelude::*;
use wall_e::diff_drive_model::DiffDriveModel;
use wall_e::fcn::*;
use wall_e::goal::Goal;

pub struct Visualizer {
    model: DiffDriveModel,
    fcn: FCN,
    goal: Goal,
    time: usize,
}

impl From<Experiment> for Visualizer {
    fn from(ex: Experiment) -> Visualizer {
        Visualizer {
            model: DiffDriveModel::spawn_randomly(
                ex.reward.init_x_bounds(),
                ex.reward.init_y_bounds(),
                ex.reward.init_or_bounds(),
                ex.reward.radius(),
            ),
            fcn: ex.fcn,
            goal: Goal::in_region(ex.reward.goal_x_bounds(), ex.reward.goal_y_bounds()),
            time: 0,
        }
    }
}

impl event::EventHandler for Visualizer {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        let (goal_x, goal_y) = self.goal.coordinates();
        let (x, y, or_in_rad) = self.model.state();
        let control = self.fcn.at(&arr1(&[x, y, or_in_rad, goal_x, goal_y]));
        let (v, w) = (control[[0]], control[[1]]);
        self.model.set_control(v, w);
        if ((x - goal_x).powf(2.0) + (y - goal_y).powf(2.0)).sqrt() < Goal::SLACK {
        } else {
            self.model.update(0.1)?;
            self.time += 1;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        self.model.draw(ctx)?;
        self.goal.draw(ctx)?;

        let (v, w) = self.model.control();
        graphics::set_window_title(
            ctx,
            &format!(
                "fps={:.2}, v={:.2}, w={:.2}, time={:?}",
                timer::fps(ctx),
                v,
                w,
                self.time
            ),
        );
        graphics::present(ctx)
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Escape => event::quit(ctx),
            _ => (),
        }
    }
}
