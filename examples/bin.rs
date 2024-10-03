use serde::{Deserialize, Serialize};

// 状態変数 x, \dot{x}, \theta, \theta
#[derive(Serialize, Deserialize, Debug)]
struct State {
    x: f32,
    dx: f32,
    theta: f32,
    dtheta: f32,
}

fn main() {
    let s = State {
        x: 1.2,
        dx: 3.4,
        theta: 5.6,
        dtheta: 7.8,
    };

    let b = bincode::serialize(&s).unwrap();
    let s2: State = bincode::deserialize(&b).unwrap();

    println!("{:?}", b);
    println!("{:?}", s2);
}
