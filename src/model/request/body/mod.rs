mod eq;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Default, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpBodyPatternStub {
    /// strict match
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equal_to_json: Option<Value>,
    /// check request body against specified JSONPath expression
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matches_json_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_path: Option<HttpJsonPathExpressionStub>,
    /// used with [equalToJson].
    /// Relaxes the strict match by allowing extra fields in the request body.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_extra_elements: Option<bool>,
    /// used with [equalToJson].
    /// Relaxes the strict match by allowing arrays to be matched ignoring their order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_array_order: Option<bool>,
}

#[derive(Debug, Clone, Default, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpJsonPathExpressionStub {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equal_to: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matches: Option<Value>,
}

impl PartialEq for HttpBodyPatternStub {
    fn eq(&self, other: &Self) -> bool {
        self.equal_to_json
            .as_ref()
            .eq(&other.equal_to_json.as_ref())
            && self
                .ignore_extra_elements
                .as_ref()
                .eq(&other.ignore_extra_elements.as_ref())
            && self
                .ignore_array_order
                .as_ref()
                .eq(&other.ignore_array_order.as_ref())
            && self
                .matches_json_path
                .as_ref()
                .eq(&other.matches_json_path.as_ref())
            && self.json_path.as_ref().eq(&other.json_path.as_ref())
    }
}

impl PartialEq for HttpJsonPathExpressionStub {
    fn eq(&self, other: &Self) -> bool {
        self.expression.as_ref().eq(&other.expression.as_ref())
            && self.equal_to.as_ref().eq(&other.equal_to.as_ref())
            && self.contains.as_ref().eq(&other.contains.as_ref())
            && self.matches.as_ref().eq(&other.matches.as_ref())
    }
}

impl Hash for HttpBodyPatternStub {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if let Some(it) = self.equal_to_json.as_ref() {
            it.to_string().hash(state)
        }
        self.ignore_extra_elements.as_ref().hash(state);
        self.ignore_array_order.as_ref().hash(state);
    }
}
