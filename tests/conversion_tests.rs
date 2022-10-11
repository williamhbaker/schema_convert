const JSON_SCHEMA: &str = r#"{
    "$schema": "http://json-schema.org/draft-07/schema#",
    "type": "object",
    "required": ["a", "one"],
    "properties": {
      "a": {
        "type": "boolean"
      },
      "one": {
        "type": "object",
        "required": ["b"],
        "properties": {
          "b": {
            "type": "integer"
          },
          "another": {
            "type": "object",
            "properties": {
              "nested": {
                "type": "null"
              }
            }
          }
        }
      }
    }
  }"#;

#[test]
fn json_to_arrow() {
    let got = schema_convert::json_to_arrow(JSON_SCHEMA).unwrap();
    insta::assert_debug_snapshot!(got);
}
