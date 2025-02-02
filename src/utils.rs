use chrono::Local;
use percent_encoding::{percent_encode, NON_ALPHANUMERIC};
use sha1::{Digest, Sha1};

pub(crate) fn generate_file_name() -> String {
    let current_date = Local::now().format("%Y-%m-%d_%H-%M-%S");
    format!("bin_resp_{}.json", current_date)
}

pub(crate) fn calculate_sha1(data: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

pub(crate) fn encode_file_name(name: &str) -> String {
    percent_encode(name.as_bytes(), NON_ALPHANUMERIC).to_string()
}
