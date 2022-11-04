use crate::entity::Client;
use crate::repository::ClientRepository;
use crate::LimitInMemoryClientRepository;
use std::rc::Rc;
use uuid::Uuid;

// -------------------------------------------------------------------------------------------------
pub struct GetClientHandler<T: ClientRepository> {
    client_repo: Rc<T>,
}

impl<T: ClientRepository> GetClientHandler<T> {
    pub fn new(client_repo: Rc<T>) -> Self {
        Self { client_repo }
    }
    pub fn execute(&self, id: Uuid) -> Result<Client, String> {
        let client = self.client_repo.by_id(id)?;
        Ok(client)
    }
}

pub struct CreateClientHandler<T: ClientRepository> {
    client_repo: Rc<T>,
}

impl<T: ClientRepository> CreateClientHandler<T> {
    pub fn new(client_repo: Rc<T>) -> Self {
        Self { client_repo }
    }
    pub fn execute(&self, name: String, location: String) -> Uuid {
        let id = self.client_repo.next_identity();
        let client = Client::new(id, name, location);
        self.client_repo.save(client);
        id
    }
}

// -------------------------------------------------------------------------------------------------

pub struct LimitGetClientHanderV1 {
    client_repo: Rc<LimitInMemoryClientRepository>,
}

impl LimitGetClientHanderV1 {
    pub fn new(client_repo: Rc<LimitInMemoryClientRepository>) -> Self {
        Self { client_repo }
    }
    pub fn execute(&self, id: Uuid) -> Result<Client, String> {
        let client = self.client_repo.by_id(id)?;
        Ok(client)
    }
}

pub struct LimitCreateClientHandler {
    client_repo: Rc<LimitInMemoryClientRepository>,
}

impl LimitCreateClientHandler {
    pub fn new(client_repo: Rc<LimitInMemoryClientRepository>) -> Self {
        Self { client_repo }
    }
    pub fn execute(&self, name: String, location: String) -> Uuid {
        let id = self.client_repo.next_identity();
        let client = Client::new(id, name, location);
        self.client_repo.save(client);
        id
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Default)]
pub struct LimitGetClientHanderV2 {
    client_repo: Rc<LimitInMemoryClientRepository>,
}

impl LimitGetClientHanderV2 {
    pub fn new() -> Self {
        #[cfg(not(test))]
        let client_repo = Rc::new(LimitInMemoryClientRepository::new(10_usize));

        #[cfg(test)]
        let client_repo = Rc::new(LimitInMemoryClientRepository::new());

        Self { client_repo }
    }
    pub fn execute(&self, id: Uuid) -> Result<Client, String> {
        let client = self.client_repo.by_id(id)?;
        Ok(client)
    }
}

#[cfg(test)]
impl LimitGetClientHanderV2 {
    fn set_client_repo(&mut self, new_client_repo: Rc<LimitInMemoryClientRepository>) {
        self.client_repo = new_client_repo;
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use std::rc::Rc;

    use super::{GetClientHandler, LimitGetClientHanderV1, LimitGetClientHanderV2};
    use crate::entity::Client;
    use crate::repository::MockClientRepository;
    use crate::LimitInMemoryClientRepository;
    use fake::{Fake, Faker};
    use mockall::predicate;

    #[test]
    fn get_client_handler() {
        let client = Faker.fake::<Client>();
        let id = client.id();

        let mut mock_repo = MockClientRepository::new();
        mock_repo
            .expect_by_id()
            .with(predicate::eq(id))
            .times(1)
            .return_const(Ok(client.clone()));

        let get_handler = GetClientHandler::new(Rc::new(mock_repo));
        let client2 = get_handler.execute(id).unwrap();
        assert_eq!(client, client2);
    }
    #[test]
    fn limit_get_client_handler_v1() {
        let client = Faker.fake::<Client>();
        let id = client.id();

        let mut mock_repo = LimitInMemoryClientRepository::new();
        mock_repo
            .expect_by_id()
            .with(predicate::eq(id))
            .times(1)
            .return_const(Ok(client.clone()));

        let get_handler = LimitGetClientHanderV1::new(Rc::new(mock_repo));
        let client2 = get_handler.execute(id).unwrap();
        assert_eq!(client, client2);
    }

    #[test]
    fn limit_get_client_handler_v2() {
        let client = Faker.fake::<Client>();
        let id = client.id();

        let mut mock_repo = LimitInMemoryClientRepository::new();
        mock_repo
            .expect_by_id()
            .with(predicate::eq(id))
            .times(1)
            .return_const(Err("No client found for geven ID".to_string()));

        let mut get_handler = LimitGetClientHanderV2::new();
        get_handler.set_client_repo(Rc::new(mock_repo));
        let err = get_handler.execute(id);
        assert_eq!(Err("No client found for geven ID".to_string()), err);
    }
}
