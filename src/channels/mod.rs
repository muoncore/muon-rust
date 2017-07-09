
extern crate atomic_ring_buffer;

use self::atomic_ring_buffer::AtomicRingBuffer;
use self::atomic_ring_buffer::InvariantAsMut;

pub trait ChannelConnection<'channel> {

}

//pub struct Channel<'channel, GoingLeft, GoingRight> {
//  left: ChannelConnection<'channel>,
//  right: ChannelConnection<'channel>
//}

struct SimpleChannelConnection<'channel> {
  dispatch: &'channel Fn() -> (),
//  buf: Box<AtomicRingBuffer<Fn(u32) -> (), Vec<Fn(u32) -> ()>>>
}

//impl <'channel, GoingLeft, GoingRight> Channel<'channel, GoingLeft, GoingRight> {
//
//
//}
