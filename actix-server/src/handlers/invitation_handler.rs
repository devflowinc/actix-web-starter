use super::auth_handler::{AdminMember, AuthedMember};
use crate::{
    data::models::{Invitation, PgPool},
    errors::ServiceError,
    operators::{
        invitation_operator::{
            create_invitation_query, delete_invitation_by_id_query, get_invitation_by_id_query,
            get_invitations_for_organization_query, send_invitation,
        },
        org_operator::add_user_to_org_query,
        user_operator::get_user_by_email_query,
    },
};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[tracing::instrument]
pub fn email_regex() -> regex::Regex {
    regex::Regex::new(r"^[a-zA-Z0-9.!#$%&’*+/=?^_`{|}~-]+@[a-zA-Z0-9-]+(?:\.[a-zA-Z0-9-]+)*")
        .unwrap()
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct InvitationResponse {
    pub registration_url: String,
}

#[derive(Deserialize, ToSchema, Serialize, Clone, Debug)]
pub struct InvitationData {
    /// The id of the organization to invite the user to.
    pub organization_id: uuid::Uuid,
    /// The role the user will have in the organization. 0 = User, 1 = Admin, 2 = Owner.
    pub user_role: i32,
    /// The email of the user to invite. Must be a valid email as they will be sent an email to register.
    pub email: String,
    /// The url of the app that the user will be directed to in order to set their password. Usually admin.trieve.ai, but may differ for local dev or self-hosted setups.
    pub app_url: String,
    /// The url that the user will be redirected to after setting their password.
    pub redirect_uri: String,
}

/// Send Invitation
///
/// Invitations act as a way to invite users to join an organization. After a user is invited, they will automatically be added to the organization with the role specified in the invitation once they set their.
#[utoipa::path(
    post,
    path = "/invitation",
    context_path = "/api",
    tag = "invitation",
    request_body(content = InvitationData, description = "JSON request payload to send an invitation", content_type = "application/json"),
    responses(
        (status = 204, description = "Ok response. Indicates that invitation email was sent correctly."),
        (status = 400, description = "Invalid email or some other error", body = ErrorRespPayload),
    ),
    params(
        ("Organization" = String, Header, description = "The organization id to use for the request"),
    ),
    security(
        ("ApiKey" = ["admin"]),
    )
)]
#[tracing::instrument(skip(pool))]
pub async fn post_invitation(
    invitation_data: web::Json<InvitationData>,
    pool: web::Data<PgPool>,
    user: AuthedMember,
) -> Result<HttpResponse, ServiceError> {
    let invitation_data = invitation_data.into_inner();
    let email = invitation_data.email;
    if !email_regex().is_match(&email) {
        return Err(ServiceError::BadRequest("Invalid email".to_string()));
    }

    let org_role = user.role;

    if org_role < invitation_data.user_role.into() {
        return Err(ServiceError::BadRequest(
            "Can not invite user with higher role than yours".to_string(),
        ));
    }

    let existing_user_org_id = invitation_data.organization_id;
    let existing_user_role = invitation_data.user_role;

    let db_user = get_user_by_email_query(email.as_str(), pool.clone()).await?;

    let added_user_to_org = add_user_to_org_query(
        db_user.id,
        existing_user_org_id,
        existing_user_role.into(),
        pool.clone(),
    )
    .await?;

    // if added_user_to_org {
    //     return Ok(HttpResponse::NoContent().finish());
    // }

    let invitation = create_invitation(
        invitation_data.app_url,
        email.clone(),
        invitation_data.organization_id,
        invitation_data.redirect_uri,
        invitation_data.user_role,
        pool,
    )
    .await?;

    send_invitation(invitation.registration_url, invitation.invitation).await?;

    Ok(HttpResponse::NoContent().finish())
}

pub struct InvitationWithUrl {
    invitation: Invitation,
    registration_url: String,
}

#[tracing::instrument(skip(pool))]
pub async fn create_invitation(
    app_url: String,
    email: String,
    organization_id: uuid::Uuid,
    redirect_uri: String,
    user_role: i32,
    pool: web::Data<PgPool>,
) -> Result<InvitationWithUrl, ServiceError> {
    let invitation = create_invitation_query(email, organization_id, user_role, pool).await?;
    // send_invitation(app_url, &invitation)

    //TODO:figure out how to get redirect_uri
    let registration_url = format!(
        "{}/auth?inv_code={}&organization_id={}&redirect_uri={}",
        app_url, invitation.id, organization_id, redirect_uri
    );
    Ok(InvitationWithUrl {
        invitation,
        registration_url,
    })
}

/// Get Invitations
///
/// Get all invitations for the organization.
#[utoipa::path(
    get,
    path = "/invitation/{organization_id}",
    context_path = "/api",
    tag = "invitation",
    responses(
        (status = 200, description = "Invitations for the dataset", body = Vec<Invitation>),
        (status = 400, description = "Service error relating to getting invitations for the dataset", body = ErrorRespPayload),
    ),
    security(
        ("ApiKey" = ["admin"]),
    )
)]
#[tracing::instrument(skip(pool))]
pub async fn get_invitations(
    user: AdminMember,
    org_id: web::Path<uuid::Uuid>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ServiceError> {
    let invitations = get_invitations_for_organization_query(org_id.into_inner(), pool).await?;
    Ok(HttpResponse::Ok().json(invitations))
}

/// Delete Invitation
///
/// Delete an invitation by id.
#[utoipa::path(
    delete,
    path = "/invitation/{invitation_id}",
    context_path = "/api",
    tag = "invitation",
    responses(
        (status = 204, description = "Ok response. Indicates that invitation was deleted."),
        (status = 400, description = "Service error relating to deleting invitation", body = ErrorRespPayload),
    ),
    security(
        ("ApiKey" = ["admin"]),
    )
)]
#[tracing::instrument(skip(pool))]
pub async fn delete_invitation(
    user: AdminMember,
    invitation_id: web::Path<uuid::Uuid>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ServiceError> {
    let invite_id = invitation_id.into_inner();
    let invite = get_invitation_by_id_query(invite_id.clone(), pool.clone()).await?;

    delete_invitation_by_id_query(invite_id, pool).await?;
    Ok(HttpResponse::NoContent().finish())
}
