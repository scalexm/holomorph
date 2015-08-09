pub trait Chunk: Sized {
    type Msg: Send + 'static;

    fn process_msg(&mut self, msg: Msg<Self>);
}

pub enum Msg<C: Chunk> {
    PoolMsg(super::Msg),
    AreaMsg(C::Msg),
}
