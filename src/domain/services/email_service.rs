use tracing::info;

pub struct EmailService;

impl EmailService {
    pub fn new() -> Self {
        Self
    }

    pub fn validate_email(&self, _email: &str) -> bool {
        // TODO: Implement email validation logic here
        true
    }

    pub async fn send_verification_email(&self, email: String, token: String) {
        // TODO: Implement email sending logic here
        info!(
            "Sending verification email to {} with token {}",
            email, token
        );
    }
}

impl Default for EmailService {
    fn default() -> Self {
        Self::new()
    }
}
