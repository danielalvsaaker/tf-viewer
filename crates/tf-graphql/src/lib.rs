mod connection;
mod guard;

mod mutation;
mod query;

use async_graphql::{EmptyMutation, EmptySubscription};

pub type Schema = async_graphql::Schema<query::Query, EmptyMutation, EmptySubscription>;
