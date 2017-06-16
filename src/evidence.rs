// A few commonly used wrapper functions around Serde JSON
// to encapsulate common behaviour
pub mod json_ops {
use serde_json;
use serde_json::Value as Json;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
//
    pub fn from_json_or_die<T: DeserializeOwned>(j: &Json, error_msg: &str) -> T {
        let t: T = match serde_json::from_value(j.clone()) {
            Ok(t)  => t,
            Err(e) => panic!("{} {}", error_msg, e),
        };
        t
    }

    pub fn from_str_or_die<T: DeserializeOwned>
        (s: &str, error_msg: &str) -> T {
        let t: T = match serde_json::from_str(s) {
            Ok(t)  => t,
            Err(x) => {
                let panic_msg = format!("{} {}", error_msg, x);
                panic!(panic_msg)
            },
        };
        t
    }

    pub fn to_pretty_json_or_die<T: Serialize>
        (t: &T, error_msg: &str) -> String {
        match serde_json::to_string_pretty(t) {
            Ok(json_str) => json_str,
            Err(x)       => {
                let panic_msg = format!("{} {}", error_msg, x);
                panic!(panic_msg)
            },
        }
    }

    pub fn get(json: &Json, key: &str) -> String {
        if json[key].is_null() {
            "".to_owned()
      } else {
            if json[key].is_string() {
                match json[key].as_str() {
                    Some(s) => s.to_owned(),
                    None    => "".to_owned(),
                }
            } else if json[key].is_i64() {
                match json[key].as_i64() {
                  Some(i) => i.to_string(),
                    None    => "".to_owned(),
                }
            } else {
                "".to_owned()
            }
        }
    }

    #[cfg(test)]
    mod tests {
        #[derive(Debug, Deserialize, PartialEq, Serialize)]
        struct Test {
            id: u32,
            name: String
        }
        fn build_test_struct() -> Test {
            Test {
                id: 1,
                name: "Foo bar".to_owned()
            }
        }

        #[test]
        fn test_from_str_or_die_deserialize() {
            let json_str = "{\"id\":1,\"name\":\"Foo bar\"}";
            let result: Test = super::from_str_or_die(json_str,
                                                     "Doesn't matter.");
            assert_eq!(result, build_test_struct());
        }

        #[test]
        #[should_panic]
        fn test_from_str_or_die_die() -> () {
            let json_str = "This isn't valid JSON!";
            let _: Test = super::from_str_or_die(json_str,
                                                 "Doesn't matter.");
        }

        #[test]
        fn test_to_pretty_json_or_die_serialize() -> () {
            let result = super::to_pretty_json_or_die(&build_test_struct(),
                                                     "Doesn't matter.");
            let json_str = "{\n  \"id\": 1,\n  \"name\": \"Foo bar\"\n}";
            assert_eq!(result, json_str);
        }
    }
}
