use std::collections::HashMap;
use net::{Token, SessionEvent};
use super::{Session, SessionBase};
use std::collections::LinkedList;
use std::boxed::FnBox;
use std::ops::{Deref, DerefMut};

pub struct Chunk<T: Session<U>, U> {
    pub sessions: HashMap<Token, T>,
    pub impl_: U,
}

type CbContainer<T, U> = Option<LinkedList<Box<FnBox(&mut Chunk<T, U>) + Send + 'static>>>;

pub struct Ref<'a, T: Session<U> + 'a, U: 'a> {
    impl_: &'a mut U,
    callbacks: &'a mut CbContainer<T, U>,
}

impl<'a, T: Session<U>, U> Ref<'a, T, U> {
    fn new(impl_: &'a mut U, callbacks: &'a mut CbContainer<T, U>) -> Self {
        Ref {
            impl_: impl_,
            callbacks: callbacks,
        }
    }

    /* adds a callback so that a session can deal with a mutable chunk after
    returning from an handler */
    pub fn eventually<F>(&mut self, job: F)
        where F: FnOnce(&mut Chunk<T, U>) + Send + 'static {

        if self.callbacks.is_none() {
            *self.callbacks = Some(LinkedList::new());
        }

        self.callbacks.as_mut().unwrap().push_back(Box::new(job))
    }
}

impl<'a, T: Session<U>, U> Deref for Ref<'a, T, U> {
    type Target = U;

    fn deref(&self) -> &U {
        self.impl_
    }
}

impl<'a, T: Session<U>, U> DerefMut for Ref<'a, T, U> {
    fn deref_mut(&mut self) -> &mut U {
        self.impl_
    }
}

macro_rules! execute {
    ($self_: ident, $cb: ident) => {
        if let Some(callbacks) = $cb {
            for job in callbacks.into_iter() {
                job.call_box(($self_,));
            }
        }
    };
}

impl<T: Session<U>, U> Chunk<T, U> {
    pub fn new(impl_: U) -> Self {
        Chunk {
            sessions: HashMap::new(),
            impl_: impl_,
        }
    }

    pub fn process_event(&mut self, evt: SessionEvent) {
        let mut callbacks = None;

        match evt {
            SessionEvent::Connect(tok, addr) => {
                if self.sessions.contains_key(&tok) {
                    return ();
                }
                let session = T::new(SessionBase::new(tok, addr));
                let _ = self.sessions.insert(tok, session);
            }

            SessionEvent::Disconnect(tok) => {
                let session = self.sessions.remove(&tok);
                if let Some(session) = session {
                    session.close(Ref::new(&mut self.impl_, &mut callbacks));
                }
            }

            SessionEvent::Packet(tok, id, data) => {
                if let Some(session) = self.sessions.get_mut(&tok) {
                    debug!("{:?} received a packet, id {}", tok, id);

                    if let Err(err) = T::get_handler(id)(session,
                        Ref::new(&mut self.impl_, &mut callbacks), data) {

                        /* debug only because it means that some message::Deserialize
                        failed, so it is either an issue with the client or an issue
                        with Deserialize */
                        debug!("handle_packet io error: {}, id {}", err, id);
                    }
                }
            }
        }

        // we execute the callbacks left by Session
        execute!(self, callbacks);
    }

    // helper for executing a session callback
    pub fn session_callback<F>(&mut self, tok: Token, job: F)
        where F: for<'a> FnOnce(&mut T, Ref<'a, T, U>) {

        let mut callbacks = None;
        if let Some(session) = self.sessions.get_mut(&tok) {
            job(session, Ref::new(&mut self.impl_, &mut callbacks));
        }
        execute!(self, callbacks);
    }
}

// close all Sessions on dropping
impl<T: Session<U>, U> Drop for Chunk<T, U> {
    fn drop(&mut self) {
        let tokens: LinkedList<Token> = self.sessions
            .keys()
            .map(|token| *token)
            .collect();

        for tok in tokens {
            let session = self.sessions.remove(&tok).unwrap();

            let mut callbacks = None;
            session.close(Ref::new(&mut self.impl_, &mut callbacks));
            execute!(self, callbacks);
        }
    }
}
