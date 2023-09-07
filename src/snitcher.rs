use std::sync::{mpsc, Arc, Mutex};

pub trait Snitcher {
    fn get_sender(&self) -> Arc<Mutex<mpsc::Sender<String>>>;
}
