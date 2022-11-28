mod connection;
mod guard;

mod mutation;
mod query;
mod subscription;

pub type Schema =
    async_graphql::Schema<query::Query, mutation::Mutation, subscription::Subscription>;
