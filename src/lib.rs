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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_item_overwrite() {
        let i = json!({
            "test": "value"
        });
        let t = ContentType::Overwrite;
        let o = update_item(&t, &i, None);
        assert_eq!(o, Some(i.clone()));

        let o = update_item(
            &t,
            &i,
            Some(json!({
                "hello": "world"
            })),
        );
        assert_eq!(o, Some(i.clone()));
    }

    #[test]
    fn test_update_item_remove() {
        let i = json!({
            "remove": "me"
        });
        let t = ContentType::Remove;
        let o = update_item(&t, &i, None);
        assert_eq!(o, None);

        let o = update_item(
            &t,
            &i,
            Some(json!({
                "a lot of content": "to be removed"
            })),
        );
        assert_eq!(o, None);
    }

    #[test]
    fn test_update_item_merge() {
        let i = json!({
            "remove": Value::Null,
            "add": 12,
            "nested": json!({
                "remove": Value::Null,
                "change": 24,
            })
        });
        let t = ContentType::Merge;
        let o = update_item(
            &t,
            &i,
            Some(json!({
                "remove": "bye",
                "keep": "forever",
                "nested": json!({
                    "remove": json!({
                        "content": "deleted",
                    }),
                    "change": 5,
                })
            })),
        );
        assert_eq!(
            o,
            Some(json!({
                "keep": "forever",
                "add": 12,
                "nested": json!({
                    "change": 24,
                })
            }))
        )
    }
}
