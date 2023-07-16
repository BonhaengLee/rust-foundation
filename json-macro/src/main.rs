use rustc_serialize::json::Json;

macro_rules! json {
    (null) => {
        Json::Null
    }; // ([...]) => { Json::Array(...)}
       // ({...}) => { Json::Object(...)}
       // (???) => { Json::Boolean(...) };
       // (???) => { Json::Number(...) };
       // (???) => { Json::String(...) };
}

#[test]
fn json_null() {
    assert_eq!(json!(null), Json::Null);
}
