use jep106::JEP106Code;
use serde_json::json;
use std::panic::catch_unwind;

fn main() {
    let mut object = json!({});
    for bank in 0..255 {
        let mut bank_object = json!({});
        for id in 0..255 {
            match catch_unwind(|| JEP106Code::get_raw(bank, id)) {
                Ok(Some(name)) => {
                    bank_object[format!("id{}", id)] = json!(name);
                }
                _ => {}
            }
        }
        if bank_object.as_object().unwrap().len() != 0 {
            object[format!("bank{}", bank + 1)] = bank_object;
        }
    }
    println!("{}", object)
}
