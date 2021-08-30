use serial::SerialPort;
use std::time::Duration;
use std::io::{Read, Write};

fn main() {
    let mut port=open_port("/dev/ttyS0").unwrap();
    let mut req=vec![0x01,0x03,0x00,0x00,0x00,0x01];
    attach_crc(&mut req);
    port.write_all(&req).unwrap();
    let mut buf:[u8;256] = [0;256];
    loop {
        if let Ok(n) = port.read(&mut buf[..]) {
            for i in 0..n {
                print!("{:02x}", buf[i]);
            }
            println!("");
            break;
        };
    }

}

fn attach_crc(data:&mut Vec<u8>){

    let mut crc16 =crc16::State::<crc16::MODBUS>::new();
    crc16.update(&data);
    let crc16=crc16.get();
    data.push((crc16&0xff) as u8);
    data.push((crc16>>8&0xff) as u8);
}

fn open_port(port:&str)->serial::core::Result<serial::SystemPort>{
    let settings=serial::PortSettings{
        baud_rate: serial::Baud4800,
        char_size: serial::Bits8,
        parity: serial::Parity::ParityNone,
        stop_bits: serial::Stop1,
        flow_control: serial::FlowControl::FlowNone
    };
    let mut port=serial::open(&port)?;
    port.configure(&settings).unwrap();
    port.set_timeout(Duration::from_millis(1000)).unwrap();
    Ok(port)
}