use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Snap {
    id: Uuid,
    message: String,
    timestamp: chrono::DateTime<chrono::Utc>,
}

impl Snap {
    /// Create new Snap with a message.
    /// Will set current time in utc as the timestamp
    /// of the snap.
    pub fn new(message: String) -> Snap {
        Snap {
            id: Uuid::new_v4(),
            message,
            timestamp: chrono::Utc::now(),
        }
    }

    /// Getter for the snap id.
    pub fn id(&self) -> String {
        self.id.to_string()
    }

    /// Getter for the snap message.
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Getter for the snap timestamp.
    pub fn timestamp(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.timestamp
    }
}

#[cfg(test)]
mod models_test {
    use super::*;

    #[test]
    fn create_snap() {
        let message = "Test Snap";
        let snap = Snap::new(message.to_string());
        let id = snap.id();

        assert!(Uuid::parse_str(&id).is_ok());
        assert_eq!(snap.message(), message);
    }
}
