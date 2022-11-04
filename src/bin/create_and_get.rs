use mockall_tutorial::handlers::{
    CreateClientHandler, GetClientHandler, LimitCreateClientHandler, LimitGetClientHanderV1,
    LimitGetClientHanderV2,
};
use mockall_tutorial::repository_impl::InMemoryClientRepository;
use mockall_tutorial::repository_impl::LimitInMemoryClientRepository;
//use mockall_tutorial::LimitInMemoryClientRepository;
use std::rc::Rc;
use uuid::Uuid;

fn main() {
    let client_repo = Rc::new(InMemoryClientRepository::new());
    let create_handler = CreateClientHandler::new(Rc::clone(&client_repo));
    let id = create_handler.execute("Taro".to_string(), "Tokyo".to_string());
    let get_handler = GetClientHandler::new(Rc::clone(&client_repo));
    println!("{:?}", get_handler.execute(id));

    let client_repo_2 = Rc::new(LimitInMemoryClientRepository::new(10));

    let create_handler_2 = LimitCreateClientHandler::new(Rc::clone(&client_repo_2));
    let id_2 = create_handler_2.execute("Jiro".to_string(), "Saitama".to_string());
    let get_handler_2 = LimitGetClientHanderV1::new(Rc::clone(&client_repo_2));
    println!("{:?}", get_handler_2.execute(id_2));

    let id_3 = Uuid::new_v4();
    let get_handler_3 = LimitGetClientHanderV2::new();
    println!("{:?}", get_handler_3.execute(id_3));
}
