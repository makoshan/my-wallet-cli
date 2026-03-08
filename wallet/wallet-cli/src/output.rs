
use serde::Serialize;
pub fn print_result<T: Serialize>(value: &T, _json: bool) {
    println!("{}", serde_json::to_string_pretty(value).unwrap_or_default());
}
pub fn print_text(msg: &str, json: bool) {
    if json { println!("{{\"message\":\"{}\"}}", msg); } else { println!("{}", msg); }
}
