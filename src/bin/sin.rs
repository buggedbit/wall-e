use ndarray::prelude::*;

extern crate wall_e;
use wall_e::ceo::CEO;
use wall_e::fcn::*;
use wall_e::rewards::SinReward;

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
    println!("{:?}", SinReward);
    let _th_std = ceo.optimize(&mut fcn, &SinReward::reward).unwrap();

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
                "reward={}\nmodel={}\nceo={:?}\nreward={:?}",
                SinReward::reward(&fcn, &fcn.params(), ceo.num_evalation_samples),
                fcn,
                ceo,
                SinReward
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
