use crate::{
    data::models::{Company, PgPool},
    errors::ServiceError,
    prefixes::{CompanyPrefix, OrgPrefix, PrefixedUuid},
};
use actix_web::web;
use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;

#[tracing::instrument(skip(pg_pool))]
pub async fn create_company_query(
    name: String,
    org_id: PrefixedUuid<OrgPrefix>,
    pg_pool: web::Data<PgPool>,
) -> Result<Company, ServiceError> {
    use crate::data::schema::companies::dsl as company_columns;

    let mut conn = pg_pool.get().await.unwrap();

    let company = Company::from_name(name, org_id);
    let company = diesel::insert_into(company_columns::companies)
        .values(&company)
        .get_result::<Company>(&mut conn)
        .await
        .map_err(|e| {
            ServiceError::InternalServerError(
                format!("Error creating org for create_org_query: {}", e).to_string(),
            )
        })?;

    Ok(company)
}

pub async fn delete_company_query(
    company_id: PrefixedUuid<CompanyPrefix>,
    pg_pool: web::Data<PgPool>,
) -> Result<(), ServiceError> {
    use crate::data::schema::companies::dsl as company_columns;

    let mut conn = pg_pool.get().await.unwrap();

    diesel::delete(company_columns::companies.filter(company_columns::id.eq(company_id)))
        .execute(&mut conn)
        .await
        .map_err(|e| ServiceError::InternalServerError(format!("Error deleting company: {}", e)))?;

    Ok(())
}

pub async fn rename_company_query(
    company_id: PrefixedUuid<CompanyPrefix>,
    new_name: String,
    pg_pool: web::Data<PgPool>,
) -> Result<Company, ServiceError> {
    use crate::data::schema::companies::dsl as company_columns;

    let mut conn = pg_pool.get().await.unwrap();

    let company =
        diesel::update(company_columns::companies.filter(company_columns::id.eq(company_id)))
            .set(company_columns::name.eq(new_name))
            .get_result::<Company>(&mut conn)
            .await
            .map_err(|e| {
                ServiceError::InternalServerError(format!("Error renaming company: {}", e))
            })?;

    Ok(company)
}

pub async fn get_company_query(
    company_id: PrefixedUuid<CompanyPrefix>,
    pg_pool: web::Data<PgPool>,
) -> Result<Company, ServiceError> {
    use crate::data::schema::companies::dsl as company_columns;

    let mut conn = pg_pool.get().await.unwrap();

    let company = company_columns::companies
        .filter(company_columns::id.eq(company_id))
        .first::<Company>(&mut conn)
        .await
        .map_err(|e| ServiceError::InternalServerError(format!("Error getting company: {}", e)))?;

    Ok(company)
}

pub async fn list_companies_query(
    org_id: PrefixedUuid<OrgPrefix>,
    pg_pool: web::Data<PgPool>,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<Vec<Company>, ServiceError> {
    use crate::data::schema::companies::dsl as company_columns;

    let mut conn = pg_pool.get().await.unwrap();

    let limit = limit.unwrap_or(10);
    let offset = offset.unwrap_or(0);

    let companies = company_columns::companies
        .filter(company_columns::org_id.eq(org_id))
        .select(Company::as_select())
        .limit(limit)
        .offset(offset)
        .load::<Company>(&mut conn)
        .await
        .map_err(|e| {
            ServiceError::InternalServerError(format!("Error getting companies: {}", e))
        })?;

    Ok(companies)
}
