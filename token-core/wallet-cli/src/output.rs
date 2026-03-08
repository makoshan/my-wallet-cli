use serde_json::json;

pub fn print_result(json_mode: bool, key: &str, value: serde_json::Value) {
    if json_mode {
        let output = json!({ key: value });
        println!("{}", serde_json::to_string_pretty(&output).unwrap());
    } else {
        match value {
            serde_json::Value::String(s) => println!("{}: {}", key, s),
            serde_json::Value::Object(obj) => {
                println!("{}:", key);
                for (k, v) in obj {
                    println!("  {}: {}", k, v);
                }
            }
            _ => println!("{}: {}", key, value),
        }
    }
}

pub fn print_success(json_mode: bool, message: &str) {
    if json_mode {
        let output = json!({ "status": "success", "message": message });
        println!("{}", serde_json::to_string_pretty(&output).unwrap());
    } else {
        println!("✓ {}", message);
    }
}

pub fn print_error(json_mode: bool, error: &str) {
    if json_mode {
        let output = json!({ "status": "error", "message": error });
        println!("{}", serde_json::to_string_pretty(&output).unwrap());
    } else {
        eprintln!("✗ Error: {}", error);
    }
}
