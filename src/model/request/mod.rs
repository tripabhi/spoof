use crate::core::mock::StubMappingBuilder;

mod matcher;
pub mod path;
mod query;

pub trait MockRegistrable {
    fn register(&self, builder: StubMappingBuilder) -> StubMappingBuilder;
}
