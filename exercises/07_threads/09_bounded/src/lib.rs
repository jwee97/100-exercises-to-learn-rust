// TODO: Convert the implementation to use bounded channels.
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{self, Receiver, SendError, Sender, SyncSender};
use std::thread;

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>,
}

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, SendError<String>> {
        let (response_sender, response_receiver) = mpsc::sync_channel(1);
        let _ = self.sender.send(Command::Insert {
            draft,
            response_channel: response_sender,
        });
        response_receiver.recv().unwrap()
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, SendError<String>> {
        let (response_sender, response_receiver) = mpsc::sync_channel(1);
        let _ = self.sender.send(Command::Get {
            id,
            response_channel: response_sender,
        });
        response_receiver.recv().unwrap()
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = mpsc::sync_channel(capacity);
    thread::spawn(move || server(receiver));
    TicketStoreClient { sender }
}

enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: SyncSender<Result<TicketId, SendError<String>>>,
    },
    Get {
        id: TicketId,
        response_channel: SyncSender<Result<Option<Ticket>, SendError<String>>>,
    },
}

pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                response_channel.send(Ok(id)).unwrap();
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                response_channel.send(Ok(ticket.cloned())).unwrap();
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
