use ndarray::prelude::*;
use serde::{Deserialize, Serialize};
use wall_e::ceo::Reward;
use wall_e::diff_drive_model::DiffDriveModel;
use wall_e::fcn::*;
use wall_e::goal::Goal;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DiffDriveReward {
    start_x_bounds: (f32, f32),
    start_y_bounds: (f32, f32),
    start_or_bounds: (f32, f32),
    radius: f32,
    goal_x_bounds: (f32, f32),
    goal_y_bounds: (f32, f32),
    num_episode_ticks: usize,
}

impl DiffDriveReward {
    pub fn new(
        start_x_bounds: (f32, f32),
        start_y_bounds: (f32, f32),
        start_or_bounds: (f32, f32),
        radius: f32,
        goal_x_bounds: (f32, f32),
        goal_y_bounds: (f32, f32),
        num_episode_ticks: usize,
    ) -> DiffDriveReward {
        DiffDriveReward {
            start_x_bounds: start_x_bounds,
            start_y_bounds: start_y_bounds,
            start_or_bounds: start_or_bounds,
            radius: radius,
            goal_x_bounds: goal_x_bounds,
            goal_y_bounds: goal_y_bounds,
            num_episode_ticks: num_episode_ticks,
        }
    }
    pub fn start_x_bounds(&self) -> (f32, f32) {
        self.start_x_bounds
    }
    pub fn start_y_bounds(&self) -> (f32, f32) {
        self.start_y_bounds
    }
    pub fn start_or_bounds(&self) -> (f32, f32) {
        self.start_or_bounds
    }
    pub fn radius(&self) -> f32 {
        self.radius
    }
    pub fn goal_x_bounds(&self) -> (f32, f32) {
        self.goal_x_bounds
    }
    pub fn goal_y_bounds(&self) -> (f32, f32) {
        self.goal_y_bounds
    }
}

impl Reward for DiffDriveReward {
    fn reward(&self, fcn: &FCN, params: &Array1<f32>, num_episodes: usize) -> f32 {
        let mut cumulative_reward = 0.0;
        for _ in 0..num_episodes {
            // Set goal
            let goal_coordinates =
                Goal::in_region(self.goal_x_bounds, self.goal_y_bounds).coordinates();
            // Spawn agent
            let mut model = DiffDriveModel::spawn_randomly(
                self.start_x_bounds,
                self.start_y_bounds,
                self.start_or_bounds,
                self.radius,
                goal_coordinates,
            );
            // Start calculating reward
            let mut episode_reward = 0.0;
            for tick in 0..self.num_episode_ticks {
                // Curr state
                let (x, y, or_in_rad) = model.scaled_state();
                // Control for curr state
                let control = fcn.at_with(&arr1(&[x, y, or_in_rad]), params);
                let (v, w) = (control[[0]], control[[1]]);
                // Apply control
                model.set_control(v, w);
                model.update(0.1).unwrap();
                // Next state
                let (x, y, or_in_rad) = model.scaled_state();
                // Makes agent orient towards goal
                let (x_hat, y_hat) = {
                    let norm = (x * x + y * y).sqrt();
                    (x / norm, y / norm)
                };
                let angular_deviation = ((x_hat - or_in_rad.cos()).powf(2.0)
                    + (y_hat - or_in_rad.sin()).powf(2.0))
                .sqrt()
                    * (1.0 / (1.0 + tick as f32));
                episode_reward -= angular_deviation;
                // Removes rotational jitter
                episode_reward -= w.abs();
                // Makes agent translate towards goal
                let dist = (x * x + y * y).sqrt();
                episode_reward -= dist * 30.0;
            }
            // Makes agent reach the goal at the end of episode
            let (x, y, _or_in_rad) = model.scaled_state();
            let final_dist = (x * x + y * y).sqrt();
            episode_reward += 200.0 * (-final_dist).exp();
            // Makes agent stop at the end of episode
            let (v, w) = model.control();
            episode_reward += 200.0 * (-v.abs()).exp() * (-final_dist).exp();
            episode_reward += 200.0 * (-w.abs()).exp() * (-final_dist).exp();

            cumulative_reward += episode_reward;
        }

        let average_reward = cumulative_reward / num_episodes as f32;
        average_reward
    }
}
