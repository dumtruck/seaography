use std::collections::BTreeMap;

use async_graphql::dynamic::ResolverContext;
use sea_orm::{Condition, Value};

/// Entities and Field transformers configuration.
#[derive(Default)]
pub struct TransformersConfig {
    /// transform conditions before applying them to the query
    pub filter_conditions_transformers: BTreeMap<String, FnFilterConditionsTransformer>,
    pub mutation_input_object_transformers: BTreeMap<String, FnMutationInputObjectTransformer>,
}

/// guards are functions that receive the application context
pub type FnFilterConditionsTransformer =
    Box<dyn Fn(&ResolverContext, Condition) -> Condition + Sync + Send>;

pub type FnMutationInputObjectTransformer =
    Box<dyn Fn(&ResolverContext, BTreeMap<String, Value>) -> BTreeMap<String, Value> + Sync + Send>;
