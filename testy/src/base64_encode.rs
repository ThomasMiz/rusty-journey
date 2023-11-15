use std::io::{self, Read, Write};

const CHAR_INDICES: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
const PADDING_CHAR: u8 = b'=';

fn main() {
    let mut buf = [0 as u8; 3];
    let mut buf_out = [0 as u8; 4];
    let mut buf_len = 0;
    let mut out = io::stdout();

    for res in io::stdin().bytes() {
        match res {
            Ok(byte) => {
                buf[buf_len] = byte;
                
                buf_len += 1;
                if buf_len == buf.len() {
                    buf_out[0] = CHAR_INDICES[(buf[0] >> 2) as usize];
                    buf_out[1] = CHAR_INDICES[(((buf[0] & 0b011) << 4) | (buf[1] >> 4)) as usize];
                    buf_out[2] = CHAR_INDICES[(((buf[1] & 0b01111) << 2) | (buf[2] >> 6)) as usize];
                    buf_out[3] = CHAR_INDICES[(buf[2] & 0b0111111) as usize];
                    out.write_all(&buf_out).expect("Bruh what happen (with stdout)");
                    buf_len = 0;
                }
            },
            Err(_) => { panic!("Bruh what happen (with stdin)") },
        }
    }

    if buf_len == 1 {
        buf_out[0] = CHAR_INDICES[(buf[0] >> 2) as usize];
        buf_out[1] = CHAR_INDICES[(((buf[0] & 0b011) << 4)) as usize];
        buf_out[2] = PADDING_CHAR;
        buf_out[3] = PADDING_CHAR;
        out.write_all(&buf_out).expect("Bruh what happen (with stdout)");
    } else if buf_len == 2 {
        buf_out[0] = CHAR_INDICES[(buf[0] >> 2) as usize];
        buf_out[1] = CHAR_INDICES[(((buf[0] & 0b011) << 4) | (buf[1] >> 4)) as usize];
        buf_out[2] = CHAR_INDICES[(((buf[1] & 0b01111) << 2)) as usize];
        buf_out[3] = PADDING_CHAR;
        out.write_all(&buf_out).expect("Bruh what happen (with stdout)");
    }
}
