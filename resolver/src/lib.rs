pub mod query;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};

pub type ApiSchema = Schema<query::Query, EmptyMutation, EmptySubscription>;
