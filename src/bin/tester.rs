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

use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::sync::Mutex;
use std::rc::Rc;

use std::{thread, time};

use muon_core::channels::channel;
use muon_core::channels::Channel;

use std::path::Path;

fn main() {

  let mut channel: Arc<Channel<u32, u32>> = channel();

  let mut c = Arc::get_mut(&mut channel).unwrap();

  Rc::get_mut(&mut c.left).unwrap().receive(Rc::new(|x| println!("HELLO WOR£LD")));
  Rc::get_mut(&mut c.right).unwrap().receive(Rc::new(|x| println!("HELLO WOR£LD")));

  c.left.send(12);

//  let dispatch = Dispatcher::create();

//  dispatch(|| println!("HELLO WORLD!"));
//
//  let producer = thread::spawn(move || {
//    for x in 1..500 {
//      dispatch(|| println!("WOOTISH") );
//      thread::sleep(time::Duration::from_millis(10));
//    }
//  });


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

  /*
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
      //todo, check the return. if err, park for 1ms.
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
*/
  thread::sleep(time::Duration::from_millis(2000));

  /*

  Wrap in API to make this sane.

  Add in a MuonMessage struct and see how we can dispatch those.
     send(Box<MuonMessage>)

  In a channel execute


  fn send(msg: MuonMessage) {
     dispatcher.enqueue(|| left(MuonMessage{}))
  }

  , invoke the stored receive(Fn)

  remove the u32 param. just functions that can be dispatched without params.


   **/


}
