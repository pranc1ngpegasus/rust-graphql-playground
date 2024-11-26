use async_graphql::*;
use entity::organization::Organization;

pub struct Query;

#[Object]
impl Query {
    async fn organization(&self, _id: ID) -> Result<Option<Organization>> {
        Ok(None)
    }
}
