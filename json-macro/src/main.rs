#![recursion_limit = "256"]
use rustc_serialize::json::Json;
use std::collections::BTreeMap;

impl From<bool> for Json {
    fn from(b: bool) -> Json {
        Json::Boolean(b)
    }
}

impl From<i32> for Json {
    fn from(i: i32) -> Json {
        Json::Number(i as f64)
    }
}

impl From<String> for Json {
    fn from(s: String) -> Json {
        Json::String(s)
    }
}

impl<'a> From<&'a str> for Json {
    fn from(s: &'a str) -> Json {
        Json::String(s.to_string())
    }
}

// 12가지 수치 타입은 구현이 전부 비슷하므로 매크로를 작성해서 반복을 줄일 수 있다.
macro_rules! impl_from_num_for_json {
  ( $( $t:ident)* ) => {
    $(
      impl From<$t> for Json {
        fn from(n: $t) -> Json {
          Json::Number(n as f64)
        }
      }
    )*
  };
}

impl_from_num_for_json!(u8 i8 u16 i16 u32 i32 u64 i64 u128 i128 usize isize f32 f64);

macro_rules! json {
    // 매크로는 타입을 잘 구분하지 못한다. 수와 문자열은 처리할 수 없다.
    // (true) => {
    //   Json::Boolean(true)
    // };
    // (false) => {
    //   Json::Boolean(false)
    // };
    (null) => {
      Json::Null
    };
    // json! 매크로에 필요한 것은 토큰 트리다. 모든 JSON 값은 단일 토큰 트리다.
    // 수, 문자열, 불 값, null은 모두 단일 토큰이며, 객체와 배열은 괄호를 두르고 있다.
    ([ $( $element:tt ),* ]) => {
      Json::Array(vec![ $( json!($element) ),* ])
    };
    ({ $( $key:tt : $value:tt ),* }) => {
      // 임시 벡터를 없애는 코드를 작성했을 때 맵을 fields에 담아야 한다. 그러나
      // json! 호출 코드에서 우연히 fields를 쓰면 서로 다른 곳을 가리키는 두 코드가 모여있게 된다.
      // 그러나 러스트가 알아서 이름을 바꿔준다. === 위생적 매크로(hygienic macro)
        {
          let mut fields = Box::new(HashMap::new());
          $( fields.insert($key.to_string(), json!($value)); )*
          Json::Object(fields)
        }
    };
    ( $other:tt ) => {
      Json::from($other)
    }
}

#[test]
fn test_json_null() {
    assert_eq!(json!(null), Json::Null);
}

#[test]
fn json_array_with_json_element() {
    let macro_generated_value = json!([
      // `$element:expr`에 매칭되지 않는 유효한 JSON.
      {
        "pitch": 440.0
      }
    ]);
    let hand_coded_value = Json::Array(vec![Json::Object(Box::new(
        vec![("pitch".to_string(), Json::Number(440.0))]
            .into_iter()
            .collect(),
    ))]);
    assert_eq!(macro_generated_value, hand_coded_value);
}
