

use Muon;
use std::sync::Arc;
use std::sync::Mutex;

pub struct RpcClient<'muon> {
  pub muon: Arc<Mutex<Muon<'muon>>>
}

pub struct RpcServer {

}

impl <'muon> RpcServer {
  pub fn create(muon: Arc<Mutex<Muon<'muon>>>) -> Box<RpcServer> {

    Box::new(RpcServer {

    })
  }
}

impl <'muon> RpcClient<'muon> {
  pub fn create(muon: Arc<Mutex<Muon<'muon>>>) -> Box<RpcClient<'muon>> {
    Box::new(RpcClient {
      muon: muon.clone()
    })
  }
}
