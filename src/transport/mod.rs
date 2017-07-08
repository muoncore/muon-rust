
pub mod inmem;

pub trait Transport<'muon> {
  fn shutdown(&mut self);
}
