use juniper::{Context, EmptyMutation, EmptySubscription, RootNode, graphql_object};

pub struct GraphQLContext();

impl GraphQLContext {
    pub fn new() -> Self {
        Self()
    }
}

impl Context for GraphQLContext {}

#[derive(Clone, Copy, Debug)]
pub struct Query;

/// The root query object of the schema
#[graphql_object(context = GraphQLContext)]
impl Query {
    fn hello_world(#[graphql(context)] _ctx: &GraphQLContext) -> Option<String> {
        Some(format!("Hello, {}!", "world"))
    }
}

pub type Schema =
    RootNode<'static, Query, EmptyMutation<GraphQLContext>, EmptySubscription<GraphQLContext>>;

pub fn schema() -> Schema {
    Schema::new(Query, EmptyMutation::new(), EmptySubscription::new())
}
