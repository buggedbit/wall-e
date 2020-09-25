use ndarray::prelude::*;

extern crate wall_e;
use rand::Rng;
use wall_e::ceo::CEO;
use wall_e::fcn::*;

pub fn reward(fcn: &FCN, params: &Array1<f32>, num_samples: usize) -> f32 {
    let mut rng = rand::thread_rng();
    let max_x = 6.28;
    let cumulative_reward = (0..num_samples)
        .map(|_| {
            let x = rng.gen::<f32>() * max_x;
            let y_true = x.sin();
            let y_pred = fcn.at_with(&arr1(&[x]), &params)[0];
            -(y_true - y_pred) * (y_true - y_pred)
        })
        .sum::<f32>();
    cumulative_reward / num_samples as f32
}

fn main() {
    let mut fcn = FCN::new(vec![
        (1, Activation::Linear),
        (5, Activation::Sigmoid),
        (5, Activation::Sigmoid),
        (5, Activation::Sigmoid),
        (1, Activation::Linear),
    ]);
    println!("{}", fcn);
    let ceo = CEO::default();
    println!("{:?}", ceo);
    let _th_std = ceo.optimize(&mut fcn, &reward).unwrap();

    use gnuplot::*;
    let mut fg = Figure::new();
    fg.axes2d()
        .lines(
            (0..=314).map(|x| x as f32 / 50.0),
            (0..=314).map(|x| (x as f32 / 50.0).sin()),
            &[Caption("true"), LineWidth(1.0), Color("green")],
        )
        .lines(
            (0..=314).map(|x| x as f32 / 50.0),
            (0..=314).map(|x| fcn.at(&arr1(&[x as f32 / 50.0]))[[0]]),
            &[Caption("pred"), LineWidth(1.0), Color("red")],
        )
        .set_legend(
            Graph(0.5),
            Graph(0.9),
            &[Placement(AlignCenter, AlignTop)],
            &[TextAlign(AlignRight)],
        )
        .set_grid_options(true, &[LineStyle(SmallDot), Color("black")])
        .set_x_grid(true)
        .set_y_grid(true)
        .set_title(
            &format!(
                "reward={}\nmodel={}\nceo={:?}\n",
                reward(&fcn, &fcn.params(), ceo.num_evalation_samples),
                fcn,
                ceo,
            ),
            &[],
        );
    let now = chrono::offset::Local::now();
    fg.save_to_png(format!("sin:{},{}.png", now.date(), now.time()), 800, 500)
        .unwrap();

    use std::fs::File;
    serde_json::to_writer(
        &File::create(format!("sin:{},{}.json", now.date(), now.time())).unwrap(),
        &fcn,
    )
    .unwrap();
}
