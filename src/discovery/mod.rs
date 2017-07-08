
pub mod inmem;

pub struct InstanceDescriptor {
  pub id: String,
  pub identifier: String,
  pub tags: Vec<String>,
  pub codecs: Vec<String>,
  pub connection_urls: Vec<String>
}

pub struct ServiceDescriptor {
  pub id: String,
  pub identifier: String,
  pub tags: Vec<String>,
  pub codecs: Vec<String>,
  pub instances: Vec<InstanceDescriptor>
}

pub trait Discovery<'muon> {
//  fn on_ready<F>(&self, func: F);
  fn advertise_local_service(&mut self, descriptor: InstanceDescriptor);
  fn shutdown(&mut self);
  fn get_known_services(&self) -> Vec<&'muon str>;
  fn find_service(&self, name: &str) -> ServiceDescriptor;
}
