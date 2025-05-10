use actix_web::guard::{Guard, GuardContext};
use dto::user::{User, UserRole};

pub struct UserRoleGuard {
    allowed_roles: &'static [UserRole],
}

impl UserRoleGuard {
    pub fn new(allowed_roles: &'static [UserRole]) -> Self {
        Self {
            allowed_roles,
        }
    }
}

impl Guard for UserRoleGuard {
    #[tracing::instrument(name = "role_guard", skip_all, level = "info")]
    fn check(&self, ctx: &GuardContext<'_>) -> bool {
        ctx.req_data()
            .get::<User>()
            .map(|u| self.allowed_roles.contains(&u.role))
            .unwrap_or(false)
    }
}
