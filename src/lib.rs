pub mod schema;

pub mod connection;

pub mod auth;

pub mod users {
    pub mod create_user;
    pub mod delete_user;
    pub mod read_user;
    pub mod update_user;
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
        (
            r"[a-z]",
            "Password must be at last one lowercase letter",
        ),
        (
            r"[A-Z]",
            "Password must be at last one uppercase letter",
        ),
        (r"\d", "Password must be at last one digit"),
        (
            r"[@$!%*?&]",
            "Password must be at last one special character -> (@$!%*?&)",
        ),
        (
            r".{8,}",
            "Password must be at least 8 characters long",
        ),
    ];
}

pub mod validators {
    use regex::Regex;
    use crate::constants::RULES;

    #[derive(Debug)]
    pub enum PasswordErrors {
        EmptyPassword,
        InvalidPassword(String),
        RegexError(String),
    }

    impl PasswordErrors {
        pub fn to_string(&self) -> String {
            match self {
                PasswordErrors::EmptyPassword => "Password cannot be empty".to_string(),
                PasswordErrors::InvalidPassword(e) => e.to_string(),
                PasswordErrors::RegexError(e) => e.to_string(),
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
    pub enum EmailErrors {InvalidEmail,
        RegexError(String),
    }

    impl EmailErrors {
        pub fn to_string(&self) -> String {
            match self {
                EmailErrors::InvalidEmail => "Invalid e-mail format.".to_string(),
                EmailErrors::RegexError(e) => e.to_string(),
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
