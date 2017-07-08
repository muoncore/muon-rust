extern crate muon_core;

use muon_core::discovery::Discovery;
use muon_core::discovery::inmem::InMemDiscovery;
use muon_core::discovery::InstanceDescriptor;
use muon_core::discovery::ServiceDescriptor;
use muon_core::transport::Transport;
use muon_core::transport::inmem::InMemTransport;
use muon_core::MuonLib;
use muon_core::MuonConfigurer;
use muon_core::Muon;

use muon_core::stack::rpc::RpcClient;

use std::sync::Arc;
use std::sync::Mutex;

fn main() {

  let mut disco = Box::new(InMemDiscovery {});
  let mut transport = Box::new(InMemTransport {});
  let mut muon = MuonLib::create(disco, transport);

  let muonref = Arc::new(Mutex::new(muon));

  let rpc = RpcClient::create(muonref);

  {
    let mut othermuon = rpc.muon.lock().unwrap();
    othermuon.disco();
  }
}
