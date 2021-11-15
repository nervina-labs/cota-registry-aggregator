use jsonrpc_http_server::jsonrpc_core::Value;

fn remove_0x(str: &str) -> &str {
    if str.contains("0x") {
        &str[2..]
    } else {
        str
    }
}

pub fn values_to_u8_vec(values: Vec<Value>) -> Vec<[u8; 32]> {
    values.iter().map(|value| {
        let str = value.as_str().unwrap();
        let hex = remove_0x(str);
        let hex_vec = hex::decode(hex).unwrap();
        let mut hex_bytes = [0u8; 32];
        hex_bytes.copy_from_slice(&hex_vec);
        hex_bytes
    }).collect()
}

pub fn check_request_params_error(values: Vec<Value>) -> bool {
    for value in values.iter() {
        if !value.is_string() {
            return true;
        }
        let hex_str = remove_0x(value.as_str().unwrap());
        if hex_str.len() != 64 {
            return true;
        }
        if hex::decode(hex_str).is_err() {
            return true;
        }
    }
    false
}

#[test]
fn test_values_to_u8_vec() {
    let hex = "0xea28c98f38b4a57aa81756b167bb37fa42daf67edbc9863afb8172096ed301c2";
    let values = vec![Value::String(hex.to_string())];
    let vec = values_to_u8_vec(values);
    let result: Vec<[u8; 32]> = vec![[234, 40, 201, 143, 56, 180, 165, 122, 168, 23, 86, 177, 103, 187, 55, 250, 66, 218, 246, 126, 219, 201, 134, 58, 251, 129, 114, 9, 110, 211, 1, 194]];
    assert_eq!(result, vec);
}

#[test]
fn test_check_request_params() {
    let values = vec![Value::String("0xea28c98f38b4a57aa81756b167bb37fa42daf67edbc9863afb8172096ed301c2".to_string())];
    assert!(!check_request_params_error(values));

    let values = vec![Value::String("0xea28c98f38b4a57aa81756b167bb37fa42daf67edbc9863afb8172096ed301c2000000000088993355".to_string()), Value::String("0x28c98f38b4a57aa81756b167bb37fa42daf67edbc9863afb8172096e".to_string())];
    assert!(check_request_params_error(values));
}