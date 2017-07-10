
use super::atomic_ring_buffer::AtomicRingBuffer;
//use self::atomic_ring_buffer::InvariantAsMut;

//use std::sync::atomic::AtomicBool;
use std::sync::Arc;
//use std::sync::Mutex;
//use std::rc::Rc;
use std::{thread, time};

/*
 An async event dispatch function
 Based on a single consumer thread spinning on a ringbuffer.
*/

pub fn dispatch<T: FnMut()>(mut exec: T) where T: FnMut()->()+Send+Sync+'static {
  DISPATCHER.buf.enqueue(|val| *val = Arc::new(exec));
}

struct Dispatcher {
  buf: Arc<AtomicRingBuffer<Arc<FnMut()->()+Send+Sync>, Vec<Arc<FnMut()->()+Send+Sync>>>>
}

lazy_static! {
    static ref DISPATCHER: Dispatcher = {
        let mut vec: Vec<Arc<FnMut()->()+Send+Sync>> = Vec::new();
    for x in 1..1024 {
      vec.push(Arc::new(|| println!("HELLO!!")));
    }

    let mut rb = Arc::new(AtomicRingBuffer::<Arc<FnMut()->()+Send+Sync>, Vec<Arc<FnMut()->()+Send+Sync>>>::new(vec));

    let consumerbuf = rb.clone();

    let consumer = thread::spawn(move || {
      let mut c = 1;
      loop {
        c += 1;
        consumerbuf.dequeue(|x| Arc::get_mut(x).unwrap()());
        //todo, check the return. if err, park for 1ms.
        thread::park_timeout_ms(1);
      }
    });

    Dispatcher {
      buf: rb
    }
    };
}
