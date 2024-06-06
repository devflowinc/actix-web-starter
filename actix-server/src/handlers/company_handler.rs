use super::auth_handler::AuthedMember;
use crate::{
    data::models::PgPool,
    operators::company_operator::{
        create_company_query, delete_company_query, get_company_query, list_companies_query,
        rename_company_query,
    },
    prefixes::{CompanyPrefix, PrefixedUuid},
};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateCompanyReqPayload {
    name: String,
}

#[utoipa::path(
  post,
  path = "/companies",
  context_path = "/api",
  tag = "companies",
  request_body(content = CreateCompanyReqPayload, description = "JSON request payload to create a new company", content_type = "application/json"),
  responses(
      (status = 201, description = "JSON body representing the company that was created", body = Company),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
  ),
  params(
    ("Organization" = String, Header, description = "The organization id to use for the request"),
  ),
  security(
      ("ApiKey" = ["readonly"]),
  )
)]
#[tracing::instrument(skip(pg_pool))]
pub async fn create_company(
    req_payload: web::Json<CreateCompanyReqPayload>,
    org_member: AuthedMember,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let name = req_payload.name.clone();
    let company = create_company_query(name, org_member.org_id, pg_pool).await?;
    Ok(HttpResponse::Created().json(company))
}

#[utoipa::path(
  delete,
  path = "/companies/{company_id}",
  context_path = "/api",
  tag = "companies",
  responses(
      (status = 204, description = "No content response indicating that the company was successfully deleted"),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
  ),
  params(
    ("Organization" = String, Header, description = "The organization id to use for the request"),
    ("company_id" = String, Path, description = "The id of the company you want to delete."),
  ),
  security(
      ("ApiKey" = ["readonly"]),
  )
)]
#[tracing::instrument(skip(pg_pool))]
pub async fn delete_company(
    org_user: AuthedMember,
    company_id: web::Path<PrefixedUuid<CompanyPrefix>>,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let company_id = company_id.into_inner();

    delete_company_query(company_id, pg_pool).await?;

    return Ok(HttpResponse::NoContent().finish());
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateCompanyReqPayload {
    name: String,
}

#[utoipa::path(
  put,
  path = "/companies/{company_id}",
  context_path = "/api",
  tag = "companies",
  request_body(content = UpdateCompanyReqPayload, description = "JSON request payload to rename the company", content_type = "application/json"),
  responses(
      (status = 200, description = "Object representing the renamed note", body = Company),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
  ),
  params(
    ("Organization" = String, Header, description = "The organization id to use for the request"),
    ("company_id" = String, Path, description = "The id of the note you want to update."),
  ),
  security(
      ("ApiKey" = ["readonly"]),
  )
)]
#[tracing::instrument(skip(pg_pool))]
pub async fn update_company(
    req_payload: web::Json<UpdateCompanyReqPayload>,
    company_id: web::Path<PrefixedUuid<CompanyPrefix>>,
    org_member: AuthedMember,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let company_id = company_id.into_inner();

    let new_name = req_payload.name.clone();

    let new_company = rename_company_query(company_id, new_name, pg_pool).await?;

    Ok(HttpResponse::Ok().json(new_company))
}

#[derive(Debug, Deserialize)]
pub struct GetCompaniesQuery {
    limit: Option<i64>,
    offset: Option<i64>,
}

#[utoipa::path(
  get,
  path = "/companies",
  context_path = "/api",
  tag = "companies",
  params(
      ("limit" = Option<i64>, Query, description = "Limit the number of results. Default is 10"),
      ("offset" = Option<i64>, Query, description = "Offset the results. Default is 0"),
      ("Organization" = String, Header, description = "The organization id to use for the request"),
  ),
  responses(
      (status = 200, description = "List of companies for the organization", body = [Company]),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
  ),
  security(
      ("ApiKey" = ["readonly"]),
  )
)]
#[tracing::instrument(skip(pg_pool))]
pub async fn get_companies_for_org(
    query: web::Query<GetCompaniesQuery>,
    authed_user: AuthedMember,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let companies =
        list_companies_query(authed_user.org_id, pg_pool, query.limit, query.offset).await?;

    Ok(HttpResponse::Ok().json(companies))
}

#[utoipa::path(
  get,
  path = "/companies/{company_id}",
  context_path = "/api",
  tag = "companies",
  responses(
      (status = 200, description = "JSON object representing the requested company", body = Company),
      (status = 401, description = "Service error relating to authentication status of the user", body = ErrorRespPayload),
  ),
  params(
    ("Organization" = String, Header, description = "The organization id to use for the request"),
    ("company_id" = String, Path, description = "The id of the company you want to fetch."),
  ),
  security(
      ("ApiKey" = ["readonly"]),
  )
)]
#[tracing::instrument(skip(pg_pool))]
pub async fn get_company_by_id(
    company_id: web::Path<PrefixedUuid<CompanyPrefix>>,
    org_member: AuthedMember,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let company_id = company_id.into_inner();

    let company = get_company_query(company_id, pg_pool).await?;

    if company.org_id != org_member.org_id {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    Ok(HttpResponse::Ok().json(company))
}
