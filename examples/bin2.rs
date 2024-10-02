// 状態変数 x, \dot{x}, \theta, \theta
#[derive(Debug)]
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
    buf[0..4].copy_from_slice(&s.x.to_le_bytes());
    buf[4..8].copy_from_slice(&s.dx.to_le_bytes());
    buf[8..12].copy_from_slice(&s.theta.to_le_bytes());
    buf[12..16].copy_from_slice(&s.dtheta.to_le_bytes());

    // buf[0..4] を f32 に変換
    let x = f32::from_le_bytes(buf[0..4].try_into().unwrap());
    let dx = f32::from_le_bytes(buf[4..8].try_into().unwrap());
    let theta = f32::from_le_bytes(buf[8..12].try_into().unwrap());
    let dtheta = f32::from_le_bytes(buf[12..16].try_into().unwrap());
    let s2 = State {
        x,
        dx,
        theta,
        dtheta,
    };

    println!("{:?}", s);
    println!("{:?}", buf);
    println!("{:?}", s2);
}
