use std::io::BufRead;
use std::io::BufReader;
use std::time::Duration;
use uart_rs_test::State;
use zerocopy::FromBytes;

fn main() {
    let port = serialport::new("/dev/ttyUSB0", 115_200)
        .timeout(Duration::from_secs(1))
        .open()
        .expect("Failed to open port");

    let mut reader = BufReader::new(port);

    loop {
        // データが読み込まれるまで待機
        let mut buf = Vec::new();
        reader
            .read_until(0x00, &mut buf)
            .expect("Failed to skip until 0x00");

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
