use crate::matcher::query::QueryParamExactMatcher;
use crate::matcher::query_param;
use crate::model::request::matcher::GenericMatcherStub;
use crate::model::request::query::HttpQueryParamsStub;
use itertools::Itertools;

impl TryFrom<&HttpQueryParamsStub> for Vec<QueryParamExactMatcher> {
    type Error = ();

    fn try_from(http_query_params: &HttpQueryParamsStub) -> Result<Self, Self::Error> {
        http_query_params.get_query_params_as_iter()
            .ok_or_else(|| ())
            .map(|iter| {
                iter.filter(|it| it.is_exact_match())
                    .filter_map(|it| QueryParamExactMatcher::try_from(&it).ok())
                    .collect_vec()
        })
    }
}

impl TryFrom<&GenericMatcherStub> for QueryParamExactMatcher {
    type Error = ();

    fn try_from(query: &GenericMatcherStub) -> Result<Self, Self::Error> {
        query
            .get_equal_to_as_string()
            .map(|eq| query_param(query.key.as_str(), eq.as_str()))
            .ok_or_else(|| ())
    }
}
