fn text_to_hex(text: &str) -> String {
    let mut hex_string = String::new();
    for byte in text.as_bytes() {
        hex_string.push_str(&format!("{:02x}", byte));
    }
    hex_string
}

fn text_to_binary(text: &str) -> String {
    let mut binary_string = String::new();
    for byte in text.as_bytes() {
        // Format each byte as an 8-bit binary string, left-padded with zeros
        binary_string.push_str(&format!("{:08b}", byte));
    }
    binary_string
}

pub fn obscure(prtgn_text:String) -> String {
    let text = prtgn_text.as_str();
    let hex = text_to_hex(text);
    let obscured = text_to_binary(hex.as_str());
    return(obscured);
}

fn decrypt() {
    
}