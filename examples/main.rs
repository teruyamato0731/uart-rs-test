use std::io::{BufRead, BufReader};
use std::time::Duration;
use uart_rs_test::{Control, State};
use zerocopy::{AsBytes, FromBytes};

fn main() {
    let mut port = serialport::new("/dev/ttyUSB0", 115_200)
        .timeout(Duration::from_millis(10))
        .open()
        .expect("Failed to open port");

    let mut reader = BufReader::new(port.try_clone().unwrap());

    loop {
        let c = Control { u: 1234 };
        let buf = c.as_bytes();
        let cobs: [u8; 4] = cobs_rs::stuff::<2, 4>(buf[0..2].try_into().unwrap(), 0);
        port.write_all(&cobs).expect("Write failed!");

        // データが読み込まれるまで待機
        let mut buf = Vec::new();
        if let Ok(_len) = reader.read_until(0x00, &mut buf) {
            println!("{:?}", buf);
            // 18バイト読み込まれたら処理を行う
            if buf.len() == 18 {
                let (cobs, _) = cobs_rs::unstuff::<18, 16>(buf.try_into().unwrap(), 0);
                println!("{:?}", cobs);

                let s = State::ref_from(&cobs);
                println!("{:?}", s);
            }
        }
    }
}
