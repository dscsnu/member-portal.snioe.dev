use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Claims {
    #[serde(rename = "sub")]
    pub user_id: String,

    pub app_metadata: AppMetadata,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AppMetadata {
    pub custom_claims: CustomClaims,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CustomClaims {
    pub groups: Vec<Group>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Group {
    pub id: String,
    pub name: String,
    pub permissions: Vec<Permission>,
}

type Permission = String;
