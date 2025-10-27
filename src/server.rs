pub mod config;

use std::error::Error;

pub trait Server {
    fn start(&self) -> Result<(), Box<dyn Error>>;

    fn shutdown(&self) -> Result<(), Box<dyn Error>>;
}
