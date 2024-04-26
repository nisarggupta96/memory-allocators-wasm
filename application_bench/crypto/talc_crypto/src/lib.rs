// talc
use wasm_bindgen::prelude::*;

use chacha20poly1305::aead::{Aead, NewAead};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce, XChaCha20Poly1305, XNonce};

#[global_allocator]
static TALCK: talc::TalckWasm = unsafe { talc::TalckWasm::new_global() };

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn run_bench() {
    execute_bench();
}

const ITERATIONS: usize = 1000000;

pub fn execute_bench() {
    console_error_panic_hook::set_once();

    let timer = web_sys::window().unwrap().performance().unwrap();

    log(format!("Method: chacha20poly1305").as_str());
    encrypt_chacha20poly1305();
    let start = timer.now();
    for _ in 0..ITERATIONS {
        encrypt_chacha20poly1305();
    }
    let end = timer.now();
    // log durations
    let total_ms = end - start;
    let average_ms = total_ms / ITERATIONS as f64;
    log(format!("Iteration: {}", ITERATIONS).as_str());
    log(format!("  total time: {} ms", total_ms).as_str());
    log(format!("  average time for: {} ms", average_ms).as_str());

    log(format!("Method: Xchacha20poly1305").as_str());
    encrypt_xchacha20poly1305();
    let start = timer.now();
    for _ in 0..ITERATIONS {
        encrypt_xchacha20poly1305();
    }
    let end = timer.now();
    // log durations
    let total_ms = end - start;
    let average_ms = total_ms / ITERATIONS as f64;
    log(format!("Iteration: {}", ITERATIONS).as_str());
    log(format!("  total time: {} ms", total_ms).as_str());
    log(format!("  average time for: {} ms", average_ms).as_str());
}

fn encrypt_chacha20poly1305() {
    let key = Key::from_slice(b"an example very very secret key."); // 32-bytes
    let cipher = ChaCha20Poly1305::new(key);
    let nonce = Nonce::from_slice(b"unique nonce"); // 12-bytes; unique

    let msg = b"zgsodbxnkxqkaocoffvheybwcjfmtddh";

    let ciphertext = cipher
        .encrypt(nonce, msg.as_ref())
        .expect("encryption failure!");
    let plaintext = cipher
        .decrypt(nonce, ciphertext.as_ref())
        .expect("decryption failure!");
    assert_eq!(&plaintext, msg);
}

fn encrypt_xchacha20poly1305() {
    let key = Key::from_slice(b"an example very very secret key."); // 32-bytes
    let cipher = XChaCha20Poly1305::new(key);
    let nonce = XNonce::from_slice(b"extra long unique nonce!"); // 24-bytes; unique

    let msg = b"zgsodbxnkxqkaocoffvheybwcjfmtddh";

    let ciphertext = cipher
        .encrypt(nonce, msg.as_ref())
        .expect("encryption failure!");
    let plaintext = cipher
        .decrypt(nonce, ciphertext.as_ref())
        .expect("decryption failure!");
    assert_eq!(&plaintext, msg);
}
