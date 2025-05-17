/* DTOs */

use uuid::Uuid;

pub enum SignupDto {
    EmailPassord { email: String, password: String },
}

pub struct AuthUserDto {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
}
