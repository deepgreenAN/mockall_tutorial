use crate::entity::Client;
use crate::repository::ClientRepository;
use indexmap::IndexMap;
use std::cell::RefCell;
use std::collections::HashMap;
use std::default::Default;
use uuid::Uuid;

#[cfg(test)]
use mockall::automock;

#[derive(Default)]
pub struct InMemoryClientRepository {
    clients: RefCell<HashMap<Uuid, Client>>,
}

impl InMemoryClientRepository {
    pub fn new() -> Self {
        Self {
            clients: RefCell::new(HashMap::new()),
        }
    }
}

impl ClientRepository for InMemoryClientRepository {
    fn by_id(&self, id: Uuid) -> Result<Client, String> {
        match self.clients.borrow().get(&id) {
            Some(client) => Ok(client.clone()),
            None => Err("No client found for geven ID".to_string()),
        }
    }
    fn save(&self, client: Client) {
        self.clients.borrow_mut().insert(client.id(), client);
    }
    fn next_identity(&self) -> Uuid {
        Uuid::new_v4()
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Default)]
pub struct LimitInMemoryClientRepository {
    clients: RefCell<IndexMap<Uuid, Client>>,
    limit: usize,
}
impl LimitInMemoryClientRepository {
    pub fn new(limit: usize) -> Self {
        Self {
            clients: RefCell::new(IndexMap::with_capacity(limit)),
            limit,
        }
    }
}

#[cfg_attr(test, automock)]
impl LimitInMemoryClientRepository {
    pub fn by_id(&self, id: Uuid) -> Result<Client, String> {
        match self.clients.borrow().get(&id) {
            Some(client) => Ok(client.clone()),
            None => Err("No client found for geven ID".to_string()),
        }
    }
    pub fn save(&self, client: Client) {
        let mut clients = self.clients.borrow_mut();
        let clients_length = clients.len();
        if clients_length > self.limit {
            clients.shift_remove_index(0);
        }
        clients.insert(client.id(), client);
    }
    pub fn next_identity(&self) -> Uuid {
        Uuid::new_v4()
    }
}
