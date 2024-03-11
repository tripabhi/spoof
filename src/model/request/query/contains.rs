use crate::matcher::query::QueryParamContainsMatcher;
use crate::matcher::query_param_contains;
use crate::model::request::matcher::GenericMatcherStub;
use crate::model::request::query::HttpQueryParamsStub;
use itertools::Itertools;

impl TryFrom<&HttpQueryParamsStub> for Vec<QueryParamContainsMatcher> {
    type Error = ();

    fn try_from(http_query_params: &HttpQueryParamsStub) -> Result<Self, Self::Error> {
        http_query_params
            .get_query_params_as_iter()
            .ok_or_else(|| ())
            .map(|iter| {
                iter.filter(|it| it.is_contains())
                    .filter_map(|it| QueryParamContainsMatcher::try_from(&it).ok())
                    .collect_vec()
            })
    }
}

impl TryFrom<&GenericMatcherStub> for QueryParamContainsMatcher {
    type Error = ();

    fn try_from(query: &GenericMatcherStub) -> Result<Self, Self::Error> {
        query
            .value
            .as_ref()
            .filter(|_| query.is_contains())
            .and_then(|it| it.contains.as_ref())
            .map(|it| query_param_contains(query.key.as_str(), it))
            .ok_or_else(|| ())
    }
}
