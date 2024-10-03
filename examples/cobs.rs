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

    let mut buf = [0u8; 16];
    s.write_to(buf.as_mut_slice()).unwrap();
    let cobs: [u8; 18] = cobs_rs::stuff(buf, 0);
    let (mut s_buf, _): ([u8; 16], usize) = cobs_rs::unstuff(cobs, 0);
    let s2 = State::mut_from(s_buf.as_mut_slice()).unwrap();

    println!("{:?}", s);
    println!("{:?}", buf);
    println!("{:?}", cobs);
    println!("{:?}", s2);
}
