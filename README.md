# wall-e

## usage
- Install rust stable toolchain and cargo.
- Update params as needed in `src/bin/rl/main.rs`.
- Use `cargo run --release --bin rl` to run training, save experiment and visualize it.
- Use `cargo run --release --bin rl <path-to-json>` to visualize experiment.

## roadmap
- [x] Coursera lesson 1 2D kinematic modelling
- [x] Coursera lesson 2 Kinematic bicycle model
- [x] In-class activity 1 CEM fitting
- [x] In-class activity 2 Kinematic modelling
- [x] In-class activity 3 RL

- Controller impl.
    - [x] Conv, Relu needed?
        - Probably not. Since for the input (x, y, th, xg, yg) probably there no local relations or sequential memory required.
        - A simple strategy would be to orient towards goal. True theta can be obtained by a function of (x, y, xg, yg). w and v can be given to reduce theta reside and distance residue.
    - [x] Variable number of dynamic dof layers.
    - [x] Per layer activations.
    - [x] Random param initialization.
    - [x] Make ceo() a struct.
    - [x] Seperate bin crate for each network.
    - Speedup
        - [x] Parallelize.
    - [x] sin().
    - [x] exp().

- Controller design.
    - Model.
        - [x] Input design & normalization.
        - [x] Hidden layer design.
        - [x] Output design.
        - [x] Param init design.
    - Reward function.
        - [x] Different reward functions.
        - [x] Randomized start poses.
        - [x] Randomized goals.
    - Optimizor.
        - [x] CEO.
    - Scenarios
        - [x] No obstacles.
    - Goal
        - [x] Position
        - [x] Should network determine stopping condition?
    - Simulator
        - [x] Interactive differential drive model.
        - [x] Constraints on controls.
- [x] Report and demo.

- Summaries.
    - [x] LeCunn 2015 paper

- Future work
    - [ ] Hinton 2006 Paper
    - [ ] Istvan Szita and Andras Lorincz (2006)
    - [ ] Goal orientation
    - [ ] Wall boundaries.
    - [ ] Maybe move generation logic inside model? Removes into shapes a lot that way.
    - [ ] median().
    - [ ] Step level optimization vs Trajectory level optimization.
    - [ ] Known static obstacles.
