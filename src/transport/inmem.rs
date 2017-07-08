

use transport::Transport;

pub struct InMemTransport {}

impl <'muon> Transport<'muon> for InMemTransport {
  fn shutdown(&mut self) {
    println!("BUBBLE!");
  }
}
