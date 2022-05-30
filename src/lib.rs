use serde_json::{json, Value};
use strum_macros::{Display, EnumString};

#[derive(EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum ContentType {
    Merge,
    Overwrite,
    Remove,
}

fn merge(into: &mut Value, from: Value) {
    if let Value::Object(into) = into {
        if let Value::Object(from) = from {
            for (k, v) in from {
                if v.is_null() {
                    into.remove(&k);
                } else {
                    merge(into.entry(k).or_insert(Value::Null), v);
                }
            }
            return;
        }
    }
    *into = from;
}

pub fn update_item(
    content_type: &ContentType,
    content: &Value,
    current: Option<Value>,
) -> Option<Value> {
    match content_type {
        ContentType::Merge => {
            let mut next = current.unwrap_or(json!({}));
            merge(&mut next, content.clone());
            Some(next)
        }
        ContentType::Remove => None,
        ContentType::Overwrite => Some(content.clone()),
    }
}
