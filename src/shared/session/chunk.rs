use std::collections::HashMap;
use net::{Token, SessionEvent};
use super::{SessionImpl, Session};
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::collections::LinkedList;
use std::boxed::FnBox;

// base Chunk class
pub struct Chunk<T: SessionImpl<Chunk = Chunk<T, U>>, U> {
    // callbacks taking a &mut Chunk, all executed at the end of a process_event
    callbacks: RefCell<Option<LinkedList<Box<FnBox(&mut Chunk<T, U>) + Send + 'static>>>>,

    pub sessions: HashMap<Token, RefCell<Session<T>>>,
    pub impl_: U, // custom fields
}

impl<T: SessionImpl<Chunk = Chunk<T, U>>, U> Deref for Chunk<T, U> {
    type Target = U;

    #[inline(always)]
    fn deref(&self) -> &U {
        &self.impl_
    }
}

impl<T: SessionImpl<Chunk = Chunk<T, U>>, U> DerefMut for Chunk<T, U> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut U {
        &mut self.impl_
    }
}

impl<T: SessionImpl<Chunk = Chunk<T, U>>, U> Chunk<T, U> {
    pub fn new(impl_: U) -> Self {
        Chunk {
            callbacks: RefCell::new(None),
            sessions: HashMap::new(),
            impl_: impl_,
        }
    }

    pub fn process_event(&mut self, evt: SessionEvent) {
        match evt {
            SessionEvent::Connect(tok, addr) => {
                if self.sessions.contains_key(&tok) {
                    return ();
                }
                let session = Session::new(tok, addr, self);
                let _ = self.sessions.insert(tok, RefCell::new(session));
            }

            SessionEvent::Disconnect(tok) => {
                let session = self.sessions.remove(&tok);
                if let Some(session) = session {
                    session.into_inner().close(self);
                }
            }

            SessionEvent::Packet(tok, id, data) => {
                if let Some(session) = self.sessions.get(&tok) {
                    debug!("{:?} received a packet, id {}", tok, id);
                    if let Err(err) = session.borrow_mut().handle_packet(self, id, data) {

                        /* debug only because it means that some message::Deserialize failed,
                         so it is either an issue with the client or an issue with Deserialize */
                        debug!("handle_packet io error: {}, id {}", err, id);
                    }
                }
            }
        }

        // we execute the callbacks left by Session::handle_packet or Session::close
        let callbacks = self.callbacks.borrow_mut().take();
        if let Some(callbacks) = callbacks {
            for job in callbacks.into_iter() {
                job.call_box((self,));
            }
        }
    }

    // helper for executing a session callback
    pub fn session_callback<F>(&self, tok: Token, job: F)
        where F: FnOnce(&mut Session<T>, &Self) {

        if let Some(session) = self.sessions.get(&tok) {
            job(&mut session.borrow_mut(), self)
        }
    }

    // adds a callback which will be executed at the end of process_event
    pub fn eventually<F>(&self, job: F) where F: FnOnce(&mut Self) + Send + 'static {
        let mut callbacks = self.callbacks.borrow_mut();

        if callbacks.is_none() {
            *callbacks = Some(LinkedList::new());
        }

        callbacks.as_mut().unwrap().push_back(Box::new(job))
    }
}

// close all Sessions on dropping
impl<T: SessionImpl<Chunk = Chunk<T, U>>, U> Drop for Chunk<T, U> {
    fn drop(&mut self) {
        let tokens: Vec<Token> = self.sessions.values()
            .map(|session| session.borrow_mut().token)
            .collect();

        for tok in tokens {
            let session = self.sessions.remove(&tok).unwrap();
            session.into_inner().close(self);
        }
    }
}
