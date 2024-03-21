use serde_json::{Map, Value};

pub struct RelaxedJson<'a>(pub &'a Value);

impl<'a> RelaxedJson<'a> {
    fn relaxed_object_equals((a, b): (&'a Map<String, Value>, &'a Map<String, Value>)) -> bool {
        a.keys().enumerate().all(|(index, key_a)| {
            let keys_are_equal = b
                .keys()
                .nth(index)
                .map(|key_b| key_b == key_a)
                .unwrap_or_default();
            let values_are_equal = b
                .values()
                .nth(index)
                .map(|value_b| Self(&a[key_a]) == Self(value_b))
                .unwrap_or_default();
            keys_are_equal && values_are_equal
        })
    }

    fn relaxed_array_equals((a, b): (&'a Vec<Value>, &'a Vec<Value>)) -> bool {
        a.len() == b.len() && Self::is_array_inclusive(a, b)
    }

    fn is_array_inclusive(a: &'a [Value], b: &'a [Value]) -> bool {
        a.iter()
            .all(|value_a| b.iter().any(|value_b| Self(value_a) == Self(value_b)))
    }
}

impl<'a> PartialEq for RelaxedJson<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.0
            .as_object()
            .zip(other.0.as_object())
            .map(Self::relaxed_object_equals)
            .or_else(|| {
                self.0
                    .as_array()
                    .zip(other.0.as_array())
                    .map(Self::relaxed_array_equals)
            })
            .unwrap_or_else(|| self.0 == other.0)
    }
}

#[cfg(test)]
mod relaxed_json_tests {
    use super::*;
    use serde_json::json;

    fn is_equal(a: &Value, b: &Value) -> bool {
        RelaxedJson(a) == RelaxedJson(b)
    }

    #[test]
    fn extra_field_on_rhs_is_allowed() {
        let obj_a = json!({"a": 1});
        let obj_b = &json!({"a": 1, "b" : 2});
        assert!(is_equal(&obj_a, &obj_b));
    }
}
