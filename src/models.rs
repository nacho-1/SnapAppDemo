use chrono::Utc;

#[derive(Debug, Clone)]
pub struct Snap {
    id: String,
    message: String,
    timestamp: chrono::DateTime<chrono::Utc>,
}

impl Snap {
    /// Create new Snap with a message.
    /// Will set current time in utc as the timestamp
    /// of the snap.
    pub fn new(message: String) -> Snap {
        Snap {
            id: message.clone(),
            message,
            timestamp: chrono::Utc::now(),
        }
    }

    /// Getter for the snap id.
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Getter for the snap message.
    pub fn message(&self) -> &String {
        &self.message
    }

    /// Getter for the snap timestamp.
    pub fn timestamp(&self) -> &chrono::DateTime<Utc> {
        &self.timestamp
    }
}
