pub mod schema;

pub mod connection;

pub mod auth {
    pub mod jwt_auth;
    pub mod urls;
}

pub mod users {
    pub mod create_user;
    pub mod delete_user;
    pub mod read_user;
    pub mod update_user;
    pub mod urls;
    pub mod user_models;
}

pub mod constants {
    pub const ID: &str = "id";

    pub const DB_USER: &str = "DB_USER";
    pub const DB_PASS: &str = "DB_PASS";
    pub const DB_IP: &str = "DB_IP";
    pub const DB_PORT: &str = "DB_PORT";
    pub const DB_NAME: &str = "DB_NAME";

    pub const JWT_SECRET: &str = "JWT_SECRET";

    pub const BIRTH_DATE_FORMAT: &str = "%Y-%m-%d";

    pub const RULES: [(&str, &str); 5] = [
        (r"[a-z]", "Password must be at last one lowercase letter"),
        (r"[A-Z]", "Password must be at last one uppercase letter"),
        (r"\d", "Password must be at last one digit"),
        (
            r"[@$!%*?&]",
            "Password must be at last one special character -> (@$!%*?&)",
        ),
        (r".{8,}", "Password must be at least 8 characters long"),
    ];
}

pub mod validators {
    use crate::constants::RULES;
    use regex::Regex;
    use std::fmt::Display;

    #[derive(Debug)]
    pub enum PasswordErrors {
        EmptyPassword,
        InvalidPassword(String),
        RegexError(String),
    }

    impl Display for PasswordErrors {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                PasswordErrors::EmptyPassword => write!(f, "Password cannot be empty"),
                PasswordErrors::InvalidPassword(e) => write!(f, "{e}"),
                PasswordErrors::RegexError(e) => write!(f, "{e}"),
            }
        }
    }

    pub fn validate_password(password: &str) -> Result<(), PasswordErrors> {
        if password.is_empty() {
            return Err(PasswordErrors::EmptyPassword);
        }

        for (rule, err) in RULES.iter() {
            match Regex::new(rule) {
                Ok(r) => {
                    if !r.is_match(password) {
                        return Err(PasswordErrors::InvalidPassword(err.to_string()));
                    }
                }
                Err(e) => return Err(PasswordErrors::RegexError(e.to_string())),
            };
        }

        Ok(())
    }

    #[derive(Debug)]
    pub enum EmailErrors {
        InvalidEmail,
        RegexError(String),
    }

    impl Display for EmailErrors {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                EmailErrors::InvalidEmail => write!(f, "Invalid e-mail format."),
                EmailErrors::RegexError(e) => write!(f, "{e}"),
            }
        }
    }

    pub fn validate_email(e_mail: &str) -> Result<(), EmailErrors> {
        match Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$") {
            Ok(r) => {
                if !r.is_match(e_mail) {
                    return Err(EmailErrors::InvalidEmail);
                }
            }
            Err(e) => return Err(EmailErrors::RegexError(e.to_string())),
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    mod test_users;
    mod test_auth_routes;
}
