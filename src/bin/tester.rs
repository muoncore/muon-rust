extern crate muon_core;
extern crate atomic_ring_buffer;

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

use self::atomic_ring_buffer::AtomicRingBuffer;
use self::atomic_ring_buffer::InvariantAsMut;

use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::sync::Mutex;
use std::rc::Rc;

use std::{thread, time};

fn main() {

//
//  let mut disco = Box::new(InMemDiscovery {});
//  let mut transport = Box::new(InMemTransport {});
//  let mut muon = MuonLib::create(disco, transport);
//
//  let muonref = Arc::new(Mutex::new(muon));
//
//  let rpc = RpcClient::create(muonref);
//
//  {
//    let mut othermuon = rpc.muon.lock().unwrap();
//    othermuon.disco();
//  }

  let mut vec: Vec<Arc<FnMut(u32)->()+Send+Sync>> = Vec::new();

  for x in 1..1024 {
    vec.push(Arc::new(|x| println!("HELLO!!")));
  }

  println!("CAP {}", vec.len());

  let buf = Arc::new(AtomicRingBuffer::<Arc<FnMut(u32)->()+Send+Sync>, Vec<Arc<FnMut(u32)->()+Send+Sync>>>::new(vec));

  let consumerbuf = buf.clone();
  let producerbuf = buf.clone();
  let producer2buf = buf.clone();

  let consumer = thread::spawn(move || {
    let mut c = 1;
    loop {
      c += 1;
      consumerbuf.dequeue(|x| Arc::get_mut(x).unwrap()(c));
      thread::yield_now();
      thread::park_timeout_ms(1);
    }
  });

  let producer = thread::spawn(move || {
    for x in 1..500 {
      producerbuf.enqueue(|val| *val = Arc::new(|x| println!("WOOTISH {}", x)) );
      thread::sleep(time::Duration::from_millis(10));
    }
  });

  let producer2 = thread::spawn(move || {
    for x in 1..500 {
      producer2buf.enqueue(|val| *val = Arc::new(|x| println!("PRODMAN2 {}", x)) );
      thread::sleep(time::Duration::from_millis(5));
    }
  });

  thread::sleep(time::Duration::from_millis(20000));

}
