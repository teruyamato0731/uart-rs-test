use std::io::{BufRead, BufReader};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use uart_rs_test::{Control, State};
use zerocopy::{AsBytes, FromBytes};

fn main() {
    let mut port = serialport::new("/dev/ttyUSB0", 115_200)
        .timeout(Duration::from_millis(10))
        .open()
        .expect("Failed to open port");

    let mut reader = BufReader::new(port.try_clone().unwrap());
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        // データが読み込まれるまで待機
        loop {
            if let Some(s) = read(&mut reader) {
                tx.send(s).unwrap();
            }
        }
    });

    let start = std::time::Instant::now();
    loop {
        // 受信が完了するまで待機
        if let Ok(received) = rx.try_recv() {
            let elapsed = start.elapsed();
            // 時刻と受信データを表示
            println!("{:?} {:?}", elapsed, received);
        }

        // 送信する
        let c = Control { u: 1234 };
        write(&mut port, &c);

        // スリープする
        thread::sleep(Duration::from_millis(10));
    }
}

fn write(port: &mut Box<dyn serialport::SerialPort>, c: &Control) {
    const SIZE: usize = std::mem::size_of::<Control>();
    const BUF_SIZE: usize = SIZE + 2;
    let buf = c.as_bytes();
    let buf = buf[..SIZE].try_into().unwrap();
    let cobs = cobs_rs::stuff::<SIZE, BUF_SIZE>(buf, 0);
    port.write_all(&cobs).expect("Write failed!");
}

// UARTを受取り、mpscに送信する
fn read(reader: &mut BufReader<Box<dyn serialport::SerialPort>>) -> Option<State> {
    const SIZE: usize = std::mem::size_of::<State>();
    const BUF_SIZE: usize = SIZE + 2;
    let mut buf = Vec::new();
    let len = reader.read_until(0x00, &mut buf).ok()?;

    if len == BUF_SIZE {
        let buf = buf[..BUF_SIZE].try_into().ok()?;
        let (cobs, _) = cobs_rs::unstuff::<BUF_SIZE, SIZE>(buf, 0);
        State::read_from(&cobs)
    } else {
        None
    }
}
