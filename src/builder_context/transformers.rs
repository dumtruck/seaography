use std::collections::BTreeMap;

use async_graphql::dynamic::ResolverContext;
use sea_orm::Condition;

/// Entities and Field transformers configuration.
#[derive(Default)]
pub struct TransformersConfig {
    /// transform conditions before applying them to the query
    pub filter_conditions_transformers: BTreeMap<String, FnFilterConditionsTransformer>,
}

/// guards are functions that receive the application context
pub type FnFilterConditionsTransformer =
    Box<dyn Fn(&ResolverContext, Condition) -> Condition + Sync + Send>;
