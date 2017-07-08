

pub trait ChannelConnection<'channel> {

}

//pub struct Channel<'channel, GoingLeft, GoingRight> {
//  left: ChannelConnection<'channel>,
//  right: ChannelConnection<'channel>
//}

struct SimpleChannelConnection<'channel> {
  dispatch: &'channel Fn() -> ()
}

//impl <'channel, GoingLeft, GoingRight> Channel<'channel, GoingLeft, GoingRight> {
//
//
//}
