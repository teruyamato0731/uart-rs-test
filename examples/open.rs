use std::time::Duration;

fn main() {
    let mut port = serialport::new("/dev/ttyUSB0", 115_200)
        .timeout(Duration::from_millis(10))
        .open()
        .expect("Failed to open port");

    // これは送信のテストです
    let send_buf = "This is a test\n".as_bytes();
    port.write_all(send_buf).expect("Write failed!");

    loop {
        let send_buf = "Sending...\n".as_bytes();
        port.write_all(send_buf).expect("Write failed!");

        let mut serial_buf = [0; 32];
        if let Ok(_len) = port.read(serial_buf.as_mut_slice()) {
            let s: &str = std::str::from_utf8(&serial_buf).expect("Invalid UTF-8");
            print!("{}", s);
        }
    }
}
