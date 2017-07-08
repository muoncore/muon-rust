extern crate libc;
#[macro_use]
extern crate log;

mod client;
mod channels;
pub mod discovery;
pub mod transport;
pub mod stack;

use client::TransportClient;
use channels::ChannelConnection;
use transport::Transport;
use discovery::Discovery;
use discovery::InstanceDescriptor;
use discovery::ServiceDescriptor;

pub trait Muon<'muon> {
  //  fn create_client_channel(&self) -> ChannelConnection;
  // codecs
  // get disco
  // config
  fn disco(&mut self);
}

pub trait MuonConfigurer<'muon>: Muon<'muon> {
  fn add_server_stack(&mut self);
}

pub struct MuonLib<'muon> {
  pub id: String,
  discovery: Box<Discovery<'muon>>,
  transport: Box<Transport<'muon>>
}

impl <'muon> MuonLib<'muon> {
  pub fn create(disco: Box<Discovery<'muon>>, transport: Box<Transport<'muon>>) -> MuonLib<'muon> {
    MuonLib {
      id: String::from("AWESOME"),
      discovery: disco,
      transport: transport
    }
  }


}

impl <'muon> Muon<'muon> for MuonLib<'muon> {
  //  fn create_client_channel(&self) -> ChannelConnection {
  //
  //  }
  fn disco(&mut self) {
    self.discovery.shutdown();
    self.transport.shutdown();
  }
}

impl <'muon> MuonConfigurer<'muon> for MuonLib<'muon> {
  fn add_server_stack(&mut self) {}
}

/*


show how to add a client stack
how to add a server stack

server stacks
transport client

transport interface (how can we implement this using C?

*/
