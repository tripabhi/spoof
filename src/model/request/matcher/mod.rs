use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{ops::Not, str::FromStr};

pub struct GenericMatcherStub {
    pub key: String,
    pub value: Option<GenericMatcherValueStruct>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct GenericMatcherValueStruct {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equal_to: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_insensitive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matches: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absent: Option<bool>,
}

impl GenericMatcherStub {
    pub fn is_equal_to(&self) -> bool {
        self.value
            .as_ref()
            .and_then(|value| value.equal_to.as_ref())
            .is_some()
    }

    pub fn is_case_insensitive(&self) -> bool {
        self.value
            .as_ref()
            .and_then(|value| value.case_insensitive)
            .unwrap_or_default()
    }

    pub fn is_matches(&self) -> bool {
        self.value
            .as_ref()
            .and_then(|value| value.matches.as_ref())
            .is_some()
    }

    pub fn is_contains(&self) -> bool {
        self.value
            .as_ref()
            .and_then(|value| value.contains.as_ref())
            .map(|it| !it.is_empty())
            .unwrap_or_default()
            && self.is_equal_to().not()
    }

    pub fn is_absent(&self) -> bool {
        self.value
            .as_ref()
            .map(|value| value.absent.is_some())
            .unwrap_or_default()
    }

    pub fn is_exact_match(&self) -> bool {
        self.is_equal_to() && !self.is_case_insensitive() && !self.is_contains()
    }

    pub fn get_equal_to_as_string(&self) -> Option<String> {
        self.value.as_ref()?.equal_to.as_ref().and_then(|value| {
            value
                .as_str()
                .map(ToString::to_string)
                .or_else(|| value.as_bool().map(|v| v.to_string()))
                .or_else(|| value.as_i64().map(|v| v.to_string()))
        })
    }

    pub fn get_matches_as_str(&self) -> Option<&str> {
        self.value.as_ref()?.matches.as_ref()?.as_str()
    }

    pub fn get_matches_as_regex(&self) -> Option<Regex> {
        self.get_matches_as_str()
            .and_then(|it| Regex::from_str(it).ok())
    }

    pub fn is_by_regex(&self) -> bool {
        let by_regex = self.is_matches();
        let by_equality = self.is_equal_to() || self.is_case_insensitive();
        let by_contains = self.is_contains();
        by_regex && by_equality.not() && by_contains.not()
    }
}

impl TryFrom<(&String, &Value)> for GenericMatcherStub {
    type Error = ();

    fn try_from((k, v): (&String, &Value)) -> Result<Self, Self::Error> {
        Ok(Self {
            key: k.to_owned(),
            value: serde_json::from_value(v.to_owned()).ok(),
        })
    }
}
