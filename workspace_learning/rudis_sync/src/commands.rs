//! redis命令的具体实现


use crate::RUDIS_DB;
use resp::Value;


/// 处理客户端的请求，参数是resp的解析值
pub fn process_client_request(decoded_msg: Value) -> Vec<u8> {
    let reply = if let Value::Array(v) = decoded_msg {
        match &v[0] {
            Value::Bulk(ref s) if s=="GET" || s=="get" => handle_get(v),
            Value::Bulk(ref s) if s=="SET" || s=="set" => handle_set(v),
            other => Err(Value::String(format!("{:?} is not supported as of now", other))),
        }
    } else {
        Err(Value::Error("Invalid Command".to_string()))
    };

    // 无论结果如何，Value都将被编码为Vec<u8>返回
    match reply {
        Ok(r) | Err(r) => r.encode(),
    }
}


fn handle_get(v: Vec<Value>) -> Result<Value, Value> {
    let v = v.iter().skip(1).collect::<Vec<_>>();
    if v.is_empty() {
        return Err(Value::Error("Excpected 1 argument for GET command".to_string()))
    }
    let db_ref = RUDIS_DB.lock().unwrap();
    let reply = if let Value::Bulk(ref s) = &v[0] {
        db_ref.get(s).map(|e| Value::Bulk(e.to_string())).unwrap_or(Value::Null)
    } else {
        Value::Null
    };

    Ok(reply)
}

fn handle_set(v:Vec<Value>) -> Result<Value, Value> {
    // unimplemented!()
    let v = v.iter().skip(1).collect::<Vec<_>>();
    if v.is_empty() || v.len()<2 {
        return Err(Value::Error("Expected 2 arguments for SET command".to_string()))
    }
    match (&v[0], &v[1]) {
        (Value::Bulk(k), Value::Bulk(v)) => {
            let _ = RUDIS_DB
            .lock()
            .unwrap()
            .insert(k.to_string(), v.to_string());
        },
        _ => unimplemented!("SET not implemented for {:?}", v),
    }

    Ok(Value::String("OK".to_string()))
}