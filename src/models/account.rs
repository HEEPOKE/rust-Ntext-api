use crate::schemas::schemas::account;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(
    Debug,
    PartialEq,
    Eq,
    Queryable,
    Selectable,
    Identifiable,
    Serialize,
    Deserialize,
    ToSchema,
    Validate,
    Clone,
)]
#[diesel(table_name = account)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Account {
    #[schema(example = 1)]
    pub id: i32,
    #[schema(example = "account")]
    pub username: String,
    #[schema(example = "account@ex.com")]
    #[validate(email)]
    pub email: String,
    pub password: Option<String>,
    #[schema(example = "0999999999")]
    pub tel: Option<String>,
    #[schema(example = "2022-01-01 00:00:00")]
    pub created_at: NaiveDateTime,
    #[schema(example = "2022-01-01 00:00:00")]
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, PartialEq, Insertable, Serialize, Deserialize, ToSchema, Validate)]
#[diesel(table_name = account)]
pub struct NewAccount<'a> {
    #[schema(example = "account", required = true)]
    #[validate(length(min = 1, max = 255))]
    pub username: &'a str,
    #[schema(example = "account@ex.com", required = true)]
    #[validate(email)]
    pub email: &'a str,
    #[schema(example = "111111", required = false)]
    pub password: Option<&'a str>,
    #[schema(example = "0999999999", required = false)]
    pub tel: Option<&'a str>,
}

#[derive(Debug, Insertable, Serialize, Deserialize, ToSchema, Validate)]
#[diesel(table_name = account)]
pub struct CreateAccountRequest {
    #[schema(example = "account")]
    #[validate(length(min = 1, max = 255))]
    pub username: String,
    #[schema(example = "account@ex.com")]
    #[validate(email)]
    pub email: String,
    #[schema(example = "111111")]
    pub password: Option<String>,
    #[schema(example = "0999999999")]
    pub tel: Option<String>,
}

#[derive(AsChangeset, ToSchema, Validate)]
#[diesel(table_name = account)]
pub struct UpdatedAccount<'a> {
    #[schema(example = "account")]
    #[validate(length(min = 1, max = 255))]
    pub username: &'a str,
    #[schema(example = "account@ex.com")]
    #[validate(email)]
    pub email: &'a str,
    #[schema(example = "111111")]
    pub password: Option<&'a str>,
    #[schema(example = "0999999999")]
    pub tel: Option<&'a str>,
}
