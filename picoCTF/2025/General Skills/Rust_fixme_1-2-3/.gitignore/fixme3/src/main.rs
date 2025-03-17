use xor_cryptor::XORCryptor;

fn decrypt(encrypted_buffer: Vec<u8>, borrowed_string: &mut String) {
    // Key for decryption
    let key = String::from("CSUCKS");

    // Append message to the borrowed string
    borrowed_string.push_str("PARTY FOUL! Here is your flag: ");

    // Create decryption object
    let res = XORCryptor::new(&key);
    if res.is_err() {
        return; // Exit if XORCryptor creation fails
    }
    let xrc = res.unwrap();

    // Decrypt the encrypted buffer
    let decrypted_buffer = xrc.decrypt_vec(encrypted_buffer);

    // Convert decrypted bytes to string and append to borrowed_string
    borrowed_string.push_str(&String::from_utf8_lossy(&decrypted_buffer));

    // Print the final string
    println!("{}", borrowed_string);
}

fn main() {
    // Encrypted flag values as hexadecimal strings
    let hex_values = [
        "41", "30", "20", "63", "4a", "45", "54", "76", "12", "90", "7e", "53", "63", "e1", "01",
        "35", "7e", "59", "60", "f6", "03", "86", "7f", "56", "41", "29", "30", "6f", "08", "c3",
        "61", "f9", "35",
    ];

    // Convert hex strings to bytes
    let encrypted_buffer: Vec<u8> = hex_values
        .iter()
        .map(|&hex| u8::from_str_radix(hex, 16).unwrap())
        .collect();

    // Initialize the mutable string
    let mut party_foul = String::from("Using memory unsafe languages is a: ");

    // Decrypt and print the flag
    decrypt(encrypted_buffer, &mut party_foul);
}