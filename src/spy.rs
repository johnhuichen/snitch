pub mod tcp_spy;

pub trait Spy {
    fn get_message(&mut self) -> Option<String>;
}
