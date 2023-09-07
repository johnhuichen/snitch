use crate::config::Config;
use crate::snitcher::Snitcher;

mod tcp_spy;

pub trait Spy {
    fn new(config: &Config) -> Self;
    fn spy_for(&self, snitcher: &impl Snitcher);
}
