use serenity::{model::prelude::*, prelude::*};

pub struct Author(User);
impl Author {
    pub fn new(user: User) -> Self {
        Author(user)
    }
    pub async fn has_permission(
        &self,
        ctx: &Context,
        guild_id: GuildId,
        role_id: RoleId,
    ) -> Result<bool, SerenityError> {
        Ok(self.0.has_role(&ctx, guild_id, role_id).await?)
    }
}
