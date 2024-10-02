// 状態変数 x, \dot{x}, \theta, \theta
#[derive(Debug)]
struct State {
    x: f32,
    dx: f32,
    theta: f32,
    dtheta: f32,
}

impl State {
    fn to_bytes(&self) -> [u8; 16] {
        let mut buf = [0u8; 16];
        buf[0..4].copy_from_slice(&self.x.to_le_bytes());
        buf[4..8].copy_from_slice(&self.dx.to_le_bytes());
        buf[8..12].copy_from_slice(&self.theta.to_le_bytes());
        buf[12..16].copy_from_slice(&self.dtheta.to_le_bytes());
        buf
    }

    fn from_bytes(buf: [u8; 16]) -> Self {
        let x = f32::from_le_bytes(buf[0..4].try_into().unwrap());
        let dx = f32::from_le_bytes(buf[4..8].try_into().unwrap());
        let theta = f32::from_le_bytes(buf[8..12].try_into().unwrap());
        let dtheta = f32::from_le_bytes(buf[12..16].try_into().unwrap());
        State {
            x,
            dx,
            theta,
            dtheta,
        }
    }
}

fn main() {
    let s = State {
        x: 1.2,
        dx: 3.4,
        theta: 5.6,
        dtheta: 7.8,
    };

    let buf = s.to_bytes();
    let s2 = State::from_bytes(buf);

    println!("{:?}", s);
    println!("{:?}", buf);
    println!("{:?}", s2);
}
