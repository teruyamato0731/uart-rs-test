use zerocopy::{AsBytes, FromBytes, FromZeroes};

// 状態変数 x, \dot{x}, \theta, \theta
#[derive(Debug, AsBytes, FromBytes, FromZeroes)]
#[repr(C)]
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

    let b = s.as_bytes();
    let s2: State = State::read_from(b).unwrap();

    println!("{:?}", b);
    println!("{:?}", s2);
}
