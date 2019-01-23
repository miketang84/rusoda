//! Util must be keeping not related to Model.
use std::fmt::Write;
use rand::{thread_rng, Rng};
use tiny_keccak::Keccak;
use ammonia::clean;
use comrak::{markdown_to_html, ComrakOptions};

#[inline]
pub fn random_string(limit: usize) -> String {
    thread_rng().gen_ascii_chars().take(limit).collect()
}

/// Convert text to `sha3_256` hex
#[inline]
pub fn sha3_256_encode(s: &str) -> String {
    let mut sha3 = Keccak::new_sha3_256();
    sha3.update(s.as_ref());
    let mut res: [u8; 32] = [0; 32];
    sha3.finalize(&mut res);
    let mut hex = String::with_capacity(64);
    for byte in &res {
        write!(hex, "{:02x}", byte).expect("Can't fail on writing to string");
    }
    hex
}

#[inline]
pub fn markdown_render(md: &str) -> String {
    let option = ComrakOptions {
        ext_strikethrough: true,
        ext_table: true,
        ext_tasklist: true,
        ext_superscript: true,
        ..ComrakOptions::default()
    };
    clean(&markdown_to_html(md, &option))
}
