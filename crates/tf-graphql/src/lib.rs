mod connection;
mod guard;

mod mutation;
mod query;

use async_graphql::EmptySubscription;

pub type Schema = async_graphql::Schema<query::Query, mutation::Mutation, EmptySubscription>;
