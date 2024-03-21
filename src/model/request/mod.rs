use crate::core::mock::StubMappingBuilder;

mod matcher;
pub mod path;
mod query;
mod body;

pub trait MockRegistrable {
    fn register(&self, builder: StubMappingBuilder) -> StubMappingBuilder;
}
