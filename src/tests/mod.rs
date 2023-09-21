mod client_init_tests;
mod user_unauth_tests;
mod group_auth_tests;
mod auto_reauth;
mod user_auth_tests;
mod group_unauth_tests;

use std::fs::{canonicalize, File};
use std::io::Read;
use std::path::Path;

#[inline]
fn get_cookie() -> String {
    let path = canonicalize(Path::new("resources/private/cookie.txt"))
        .expect("Could not canonicalize file.");

    let mut handle = File::open(path).expect("Could not open file.");

    let len = handle
        .metadata()
        .expect("Could not get file metadata")
        .len();

    let mut buf = String::with_capacity(len as usize);

    handle
        .read_to_string(&mut buf)
        .expect("Failed to read from file.");

    drop(handle);

    let buf = buf.trim();

    buf.to_string()
}
