macro_rules! json {
  (null) => { Json::Null };
  ([...]) => { Json::Array(...)}
  ({...}) => { Json::Object(...)}
  (???) => { Json::Boolean(...) };
  (???) => { Json::Number(...) };
  (???) => { Json::String(...) };
}
