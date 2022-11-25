use state::Container;

pub mod api;
pub mod appliaction;
pub mod infrastructure;

#[macro_use]
extern crate getset;

pub static APP_CONTENT: Container![Sync + Send] = <Container![Sync + Send]>::new();
