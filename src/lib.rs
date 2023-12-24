use indexmap::IndexMap;
use serde_json::{from_str, Result, Value};

pub fn parse(json: &str) -> Result<String> {
	let json: IndexMap<String, Value> = from_str(json)?;
	let mut lua = String::from("{\n");

	for (key, value) in json {
		lua.push_str(&walk(Some(&validate_string(&key)), &value, 1));
	}

	lua.push('}');

	Ok(lua)
}

fn walk(key: Option<&str>, value: &Value, depth: usize) -> String {
	let mut lua = String::new();

	lua.push_str(&get_indent(depth));

	if let Some(key) = key {
		lua.push_str(&format!("[\"{}\"] = ", validate_string(key)));
	}

	match value {
		Value::String(s) => lua.push_str(&format!("\"{}\"", &validate_string(s))),
		Value::Number(n) => lua.push_str(&n.to_string()),
		Value::Bool(b) => lua.push_str(&b.to_string()),
		Value::Null => lua.push_str("nil"),
		Value::Array(a) => {
			lua.push_str("[\n");

			for v in a {
				lua.push_str(&walk(None, v, depth + 1));
			}

			lua.push_str(&get_indent(depth));
			lua.push(']');
		}
		Value::Object(o) => {
			lua.push_str("{\n");

			for (k, v) in o {
				lua.push_str(&walk(Some(k), v, depth + 1));
			}

			lua.push_str(&get_indent(depth));
			lua.push('}');
		}
	}

	lua.push_str(",\n");

	lua
}

fn get_indent(depth: usize) -> String {
	let mut indent = String::new();

	for _ in 0..depth {
		indent.push('\t');
	}

	indent
}

fn validate_string(string: &str) -> String {
	let mut validated = String::new();

	for char in string.chars() {
		match char {
			'\n' => validated.push_str("\\n"),
			'\t' => validated.push_str("\\t"),
			'\r' => validated.push_str("\\r"),
			'\\' => validated.push_str("\\\\"),
			'"' => validated.push_str("\\\""),
			_ => validated.push(char),
		}
	}

	validated
}

#[cfg(test)]
mod test {
	#[test]
	fn equal() {
		use crate::parse;

		let json = r#"{
  "string": "str",
  "int": 420,
  "float": 4.2,
  "bool": true,
  "null": null,
  "array": [
    "string",
    12345,
    false,
    {
      "k": "v"
    }
  ],
  "object": {
    "key": "value"
  }
}"#;

		let lua = r#"{
	["string"] = "str",
	["int"] = 420,
	["float"] = 4.2,
	["bool"] = true,
	["null"] = nil,
	["array"] = [
		"string",
		12345,
		false,
		{
			["k"] = "v",
		},
	],
	["object"] = {
		["key"] = "value",
	},
}"#;

		assert_eq!(parse(json).unwrap(), lua);
	}

	#[test]
	fn malformed_strings() {
		use crate::parse;

		let json = r#"{
  "1": "..\n..",
  "2": "..\t..",
  "3": "..\r..",
  "4": "..\\..",
  "5": "..\".."
}"#;

		let lua = r#"{
	["1"] = "..\n..",
	["2"] = "..\t..",
	["3"] = "..\r..",
	["4"] = "..\\..",
	["5"] = "..\"..",
}"#;

		assert_eq!(parse(json).unwrap(), lua);
	}
}
