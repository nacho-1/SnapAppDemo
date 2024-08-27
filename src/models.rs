use chrono::Utc;

#[derive(Debug, Clone)]
pub struct Snap {
    id: String,
    message: String,
    timestamp: chrono::DateTime<chrono::Utc>,
}

impl Snap {
    pub fn new(message: String) -> Snap {
        Snap {
            id: message.clone(),
            message,
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn message(&self) -> &String {
        &self.message
    }

    pub fn timestamp(&self) -> &chrono::DateTime<Utc> {
        &self.timestamp
    }
}
