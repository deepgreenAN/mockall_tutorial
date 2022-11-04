pub mod entity;
mod getting_started;
pub mod handlers;
pub mod repository;
pub mod repository_impl;

use mockall_double::double;

#[double]
pub use repository_impl::LimitInMemoryClientRepository;
