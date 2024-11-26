use crate::organization_id::OrganizationID;
use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};
use derive_new::new;

#[derive(Clone, Debug, new, SimpleObject)]
pub struct Organization {
    #[new(value = "OrganizationID::new()")]
    pub id: OrganizationID,
    #[new(value = "Utc::now()")]
    pub created_at: DateTime<Utc>,
}
