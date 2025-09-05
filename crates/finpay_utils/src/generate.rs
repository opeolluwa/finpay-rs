use rand::Rng;
use random_word::Lang;

pub fn generate_file_name() -> String {
    random_word::all_len(4, Lang::En).unwrap().join("_")
}

pub fn generate_otp() -> String {
    let mut rng = rand::rng();
    let otp: u32 = rng.random_range(0..=999_999);
    format!("{:06}", otp) // Always 6 digits, leading zeros preserved
}
