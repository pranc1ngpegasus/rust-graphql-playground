use async_graphql::{Object, ID};
use derive_getters::Dissolve;
use derive_new::new;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, new, Dissolve)]
pub struct OrganizationID(#[new(value = "Uuid::now_v7()")] Uuid);

#[Object]
impl OrganizationID {
    async fn id(&self) -> ID {
        ID(self.to_string())
    }
}

impl std::str::FromStr for OrganizationID {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        uuid::Uuid::parse_str(input)
            .map(Self)
            .map_err(|e| anyhow::anyhow!("invalid vault id: {}", e))
    }
}

impl From<uuid::Uuid> for OrganizationID {
    fn from(input: uuid::Uuid) -> Self {
        Self(input)
    }
}

impl std::fmt::Display for OrganizationID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<ID> for OrganizationID {
    type Error = anyhow::Error;

    fn try_from(id: ID) -> Result<Self, Self::Error> {
        Uuid::parse_str(id.to_string().as_str())
            .map(Self)
            .map_err(|e| anyhow::anyhow!("invalid vault id: {}", e))
    }
}
