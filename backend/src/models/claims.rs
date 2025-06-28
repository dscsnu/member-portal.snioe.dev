use poem_openapi::Object;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Object)]
pub struct Claims {
    #[serde(rename = "sub")]
    pub user_id: String,

    pub app_metadata: AppMetadata,
}

#[derive(Debug, Deserialize, Serialize, Clone, Object)]
pub struct AppMetadata {
    pub custom_claims: CustomClaims,
}

#[derive(Debug, Deserialize, Serialize, Clone, Object)]
pub struct CustomClaims {
    pub groups: Vec<Group>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Object)]
pub struct Group {
    pub id: String,
    pub name: String,
    pub permissions: Vec<Permission>,
}

type Permission = String;

impl Claims {
    pub fn has_permission(&self, permission: &str) -> bool {
        self.app_metadata
            .custom_claims
            .groups
            .iter()
            .any(|group| group.permissions.iter().any(|p| p == permission))
    }

    pub fn has_permissions_any(&self, permissions: &[&str]) -> bool {
        permissions.iter().any(|perm| self.has_permission(perm))
    }

    pub fn has_permissions_all(&self, permissions: &[&str]) -> bool {
        permissions.iter().all(|perm| self.has_permission(perm))
    }
}
