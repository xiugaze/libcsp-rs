
pub fn dump_buffer(buffer: &[u8], len: usize) {

    let mut hex_row = String::new();
    let mut ascii_row = String::new();

    for (i, &byte) in buffer[..len].iter().enumerate() {

        // for each row
        if i > 0 && i % 8 == 0 {
            let address_offset = i - 8;
            println!("{:08X}: {:<20} {}", address_offset, hex_row, ascii_row);
            hex_row.clear();
            ascii_row.clear();
        }

        // if we're at a multiple of 8, or at the beginning of a row
        // put the addrewss offset
        // if i % 8 == 0 {
        //     hex_row.push_str(&format!("{:08X}: ", i));
        // }

        // for each byte, push the ascii character
        hex_row.push_str(&format!("{:02X} ", byte));
        ascii_row.push(if byte.is_ascii_graphic() { byte as char } else { ' ' });
    }

    if !hex_row.is_empty() {
        let address_offset = buffer.len();
        let remaining = 8 - (len % 8);
        for _ in 0..remaining {
            hex_row.push_str("XX ");
        }

        println!("{:08X}: {:<20} {}", address_offset, hex_row, ascii_row);
    }

    println!();
}

pub fn test_buffer() -> [u8; 256] {
    let mut buffer: [u8; 256] = [0; 256];

    for i in 0..6 {
        buffer[i] = 0xFF;
    }

    let mut counter: u8 = 0x00;
    for i in 6..256 {
        buffer[i] = counter;
        if counter == 0xFF {
            counter = 0x00
        } else {
            counter += 1;
        }
    }
    buffer
}
