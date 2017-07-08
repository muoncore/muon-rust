

use discovery::Discovery;
use discovery::ServiceDescriptor;
use discovery::InstanceDescriptor;

pub struct InMemDiscovery {}

impl <'muon> Discovery<'muon> for InMemDiscovery {
  fn advertise_local_service(&mut self, descriptor: InstanceDescriptor) {
    unimplemented!()
  }

  fn shutdown(&mut self) {
    println!("CHINCKEN!");
  }

  fn get_known_services(&self) -> Vec<&'muon str> {
    unimplemented!()
  }

  fn find_service(&self, name: &str) -> ServiceDescriptor {
    unimplemented!()
  }
}
