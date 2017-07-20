
extern crate atomic_ring_buffer;
use std::rc::Rc;
use super::dispatcher::dispatch;
use std::sync::Arc;

pub trait ChannelConnection<'channel, MessageType> {
  fn send(&self, msg: MessageType);
  fn receive(&mut self, func: Rc<FnMut(MessageType)->()+Sync+Send+'static>);
}
pub struct Channel<'channel, GoingLeft, GoingRight> {
  pub left: Rc<ChannelConnection<'channel, GoingLeft>>,
  pub right: Rc<ChannelConnection<'channel, GoingRight>>,
}

pub fn channel<GoingLeft: 'static, GoingRight: 'static>() -> Arc<Channel<'static, GoingLeft,GoingRight>> {
  Arc::new(Channel {
    left: Rc::new(SimpleChannelConnection::<GoingLeft> {
      receive: None
    }),
    right: Rc::new(SimpleChannelConnection::<GoingRight> {
      receive: None
    })
  })
}

//pub struct Channel<'channel, GoingLeft, GoingRight> {
//  left: ChannelConnection<'channel>,
//  right: ChannelConnection<'channel>
//}

struct SimpleChannelConnection<MessageType> {
  receive: Option<Rc<FnMut(MessageType) ->()+Sync+Send+'static>>
//  dispatch: &'channel Fn() -> (),
//  buf: Box<AtomicRingBuffer<Fn(u32) -> (), Vec<Fn(u32) -> ()>>>
}

impl <'channel, T> ChannelConnection<'channel, T> for SimpleChannelConnection<T> {
  fn send(&self, msg: T) -> () {
//    if self.receive == None {
//
//    } else {
//      self.receive.unwrap()(msg);
//    }

//    match self.receive {
//      None => {},
//      Some(ref mut t) => {
//        let mut func = Rc::get_mut(t);
//        func(msg)
//      }
//    }
  }

  fn receive(&mut self, mut func: Rc<FnMut(T)->()+Sync+Send+'static>) {
    self.receive = Some(func.clone());
  }
}

//impl <'channel, GoingLeft, GoingRight> Channel<'channel, GoingLeft, GoingRight> {
//
//
//}
