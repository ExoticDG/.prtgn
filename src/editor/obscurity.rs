fn hex_to_binary(hex: &str) -> String {
    hex.chars()
        .map(|c| {
            let num = u8::from_str_radix(&c.to_string(), 16).unwrap();
            format!("{:04b}", num)
        })
        .collect()
}

fn binary_to_hex(binary: &str) -> String {
    binary
        .chars()
        .collect::<Vec<char>>()
        .chunks(4)
        .map(|chunk| {
            let s: String = chunk.iter().collect();
            let num = u8::from_str_radix(&s, 2).unwrap();
            format!("{:x}", num)
        })
        .collect()
}

fn hex_to_text(hex: &str) -> String {
    let mut text = String::new();
    for i in (0..hex.len()).step_by(2) {
        let byte = u8::from_str_radix(&hex[i..i+2], 16).unwrap();
        text.push(byte as char);
    }
    text
}

pub fn obscure(prtgn_text: String) -> String {
    let hex: String = prtgn_text
        .as_bytes()
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect();
    let obscured = hex_to_binary(&hex);

    obscured
}

pub fn decrypt(obscured: &str) -> String {
    let hex = binary_to_hex(obscured);
    let text = hex_to_text(&hex);

    text
}
