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

pub struct DeviceInfo {
    pub device_type: String,
    pub device_name: String,
    pub browser: String,
    pub os: String,
}
