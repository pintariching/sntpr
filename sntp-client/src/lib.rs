use std::mem;
use std::net::UdpSocket;
use std::time::{Duration, Instant, SystemTime};

mod packet;

const SNTP_TIME_OFFSET: u32 = 2_208_988_800;
const SNTP_PACKET_SIZE: usize = 48;
pub const POOL_NTP_ADDR: &str = "pool.ntp.org:123";

fn read_be_u32(input: &[u8]) -> u32 {
    let (int_bytes, _) = input.split_at(mem::size_of::<u32>());
    u32::from_be_bytes(int_bytes.try_into().unwrap())
}

pub fn request() {
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    let timeout = Some(Duration::from_secs(1));
    socket.set_read_timeout(timeout).unwrap();
    socket.set_write_timeout(timeout).unwrap();

    let mut packet = [0u8; SNTP_PACKET_SIZE];
    packet[0] = (3 << 6) | (4 << 3) | 3;

    let sent = match socket.send_to(&packet, POOL_NTP_ADDR) {
        Ok(sent) => {
            if sent != SNTP_PACKET_SIZE {
                panic!("Invalid SNTP packet size sent")
            }

            sent
        }
        Err(e) => panic!("{}", e),
    };

    println!("{}", sent);

    match socket.recv_from(&mut packet) {
        Ok((recv, _)) => {
            if recv != SNTP_PACKET_SIZE {
                panic!("Invalid SNTP packet size recieved");
            }

            let hdr = packet[0];
            let vn = (hdr & 0x38) >> 3;
            if vn != 4 {
                panic!("Server returned wrong SNTP version {vn}, expected 4");
            }

            let mode = hdr & 0x7;
            if mode != 4 && mode != 5 {
                panic!("Not a SNTP server reply");
            }

            let secs = read_be_u32(&packet[40..44]);
            let frac = read_be_u32(&packet[44..48]);

            println!("{secs}.{frac}");
        }
        Err(e) => println!("{}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request() {
        request()
    }
}
