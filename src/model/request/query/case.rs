use crate::matcher::query::{QueryParamCaseInsensitiveMatcher, QueryParamExactMatcher};
use crate::matcher::query_param_case_insensitive;
use crate::model::request::matcher::GenericMatcherStub;
use crate::model::request::query::HttpQueryParamsStub;
use itertools::Itertools;

impl TryFrom<&HttpQueryParamsStub> for Vec<QueryParamCaseInsensitiveMatcher> {
    type Error = ();

    fn try_from(http_query_params: &HttpQueryParamsStub) -> Result<Self, Self::Error> {
        http_query_params
            .get_query_params_as_iter()
            .ok_or_else(|| ())
            .map(|iter| {
                iter.filter(|it| it.is_case_insensitive())
                    .filter_map(|it| QueryParamCaseInsensitiveMatcher::try_from(&it).ok())
                    .collect_vec()
            })
    }
}

impl TryFrom<&GenericMatcherStub> for QueryParamCaseInsensitiveMatcher {
    type Error = ();

    fn try_from(query: &GenericMatcherStub) -> Result<Self, Self::Error> {
        query
            .get_equal_to_as_string()
            .filter(|_| query.is_case_insensitive())
            .map(|equal| query_param_case_insensitive(query.key.as_str(), equal))
            .ok_or_else(|| ())
    }
}
