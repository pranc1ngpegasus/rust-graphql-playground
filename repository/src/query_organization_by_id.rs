use crate::Repository;
use derive_new::new;
use entity::organization::Organization;
use entity::organization_id::OrganizationID;
use sqlx::query_file_as;
use uuid::Uuid;

#[derive(new)]
pub struct QueryOrganizationByIDImpl {}

impl Repository<Vec<OrganizationID>, Vec<Organization>> for QueryOrganizationByIDImpl {
    fn handle<'a>(
        &'a self,
        conn: &'a mut sqlx::PgConnection,
        input: Vec<OrganizationID>,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = anyhow::Result<Vec<Organization>>> + Send + 'a>,
    > {
        Box::pin(async move {
            let ids: Vec<Uuid> = input
                .iter()
                .map(|organization_id| organization_id.clone().dissolve())
                .collect();

            query_file_as!(Organization, "queries/query_organizations_by_ids.sql", &ids)
                .fetch_all(&mut *conn)
                .await
                .map_err(|e| anyhow::anyhow!("failed to fetch organizations: {}", e))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;
    use std::str::FromStr;

    #[sqlx::test(
        migrations = "../migrations",
        fixtures("../fixtures/query_organizations_by_ids.sql")
    )]
    async fn test_query_organizations_by_ids(pool: PgPool) {
        let organization_id =
            OrganizationID::from_str("01925140-736d-78b8-a86f-1585f531afcc").unwrap();

        let query_organizations_by_ids = QueryOrganizationByIDImpl::new();
        let mut conn = pool.acquire().await.unwrap();
        match query_organizations_by_ids
            .handle(&mut conn, vec![organization_id.clone()])
            .await
        {
            Ok(results) => {
                assert_eq!(results.len(), 1);
                assert_eq!(results[0].id.clone().dissolve(), organization_id.dissolve());
            }
            Err(e) => panic!("unexpected error: {}", e),
        }
    }
}
