use crate::matcher::query::QueryParamExistsMatcher;
use crate::model::request::matcher::GenericMatcherStub;
use crate::model::request::query::HttpQueryParamsStub;
use itertools::Itertools;

impl TryFrom<&HttpQueryParamsStub> for Vec<QueryParamExistsMatcher> {
    type Error = ();

    fn try_from(http_query_params: &HttpQueryParamsStub) -> Result<Self, Self::Error> {
        http_query_params
            .get_query_params_as_iter()
            .ok_or_else(|| ())
            .map(|iter| {
                iter.filter(|it| it.is_absent())
                    .filter_map(|it| QueryParamExistsMatcher::try_from(&it).ok())
                    .collect_vec()
            })
    }
}

impl TryFrom<&GenericMatcherStub> for QueryParamExistsMatcher {
    type Error = ();

    fn try_from(query: &GenericMatcherStub) -> Result<Self, Self::Error> {
        query
            .value
            .as_ref()
            .filter(|_| query.is_absent())
            .map(|it| it.absent.unwrap_or_default())
            .map(|absent| {
                if absent {
                    QueryParamExistsMatcher::does_not_exist(query.key.to_string())
                } else {
                    QueryParamExistsMatcher::does_exist(query.key.to_string())
                }
            })
            .ok_or_else(|| ())
    }
}
