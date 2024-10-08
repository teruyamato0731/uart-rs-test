use std::time::Duration;
use uart_rs_test::Control;
use zerocopy::AsBytes;

fn main() {
    let mut port = serialport::new("/dev/ttyUSB0", 115_200)
        .timeout(Duration::from_millis(10))
        .open()
        .expect("Failed to open port");

    loop {
        let c = Control { u: 1234 };
        let buf = c.as_bytes();
        let cobs: [u8; 4] = cobs_rs::stuff::<2, 4>(buf[0..2].try_into().unwrap(), 0);
        println!("{:?}", cobs);
        port.write_all(&cobs).expect("Write failed!");

        // 30ms待機
        std::thread::sleep(Duration::from_millis(30));
    }
}
