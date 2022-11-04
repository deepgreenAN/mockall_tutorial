use uuid::Uuid;

#[cfg(test)]
use fake::{Dummy, Fake};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Dummy))]
pub struct Client {
    id: Uuid,
    name: String,
    location: String,
}

impl Client {
    pub fn new(id: Uuid, name: String, location: String) -> Self {
        Self { id, name, location }
    }
    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn location(&self) -> &str {
        &self.location
    }
    pub fn edit(&mut self, name: String, location: String) {
        self.name = name;
        self.location = location;
    }
}
