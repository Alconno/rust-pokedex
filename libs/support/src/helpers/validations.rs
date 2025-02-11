use zxcvbn::zxcvbn;

/// Validate strenght of a password
pub fn password_strength(password: &str) -> bool {
    let entropy = match zxcvbn(password, &[]) {
        Ok(e) => e,
        Err(_) => return true,
    };

    entropy.score() < 3
}
