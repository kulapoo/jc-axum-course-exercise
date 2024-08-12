//! Simplistic Model Layer
//! (with mock-store layer)

use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};
use crate::{ctx::Ctx, Error, Result};


// region: Ticket types
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub title: String,
    pub cid: u64,
}


#[derive(Debug, Deserialize)]
pub struct TicketForCreate {
    pub title: String,
}

// endregion: Ticket types



// region: Ticket Controller
#[derive(Clone)]
pub struct ModelController {
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}


// constructor
impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            tickets_store: Arc::default(),
        })
    }
}


impl ModelController {
    pub async fn create_ticket(&self, ticket_fc: TicketForCreate, ctx: Ctx) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();
        let id = store.len() as u64;
        let ticket = Ticket {
            id,
            title: ticket_fc.title,
            cid: ctx.user_id(),
        };

        store.push(Some(ticket.clone()));

        Ok(ticket)
    }

    pub async fn list_tickets(&self, _ctx: Ctx) -> Result<Vec<Ticket>> {
		let store = self.tickets_store.lock().unwrap();

		let tickets = store.iter().filter_map(|t| t.clone()).collect();

		Ok(tickets)
	}

	pub async fn delete_ticket(&self, id: u64, _ctx: Ctx) -> Result<Ticket> {
		let mut store = self.tickets_store.lock().unwrap();

		let ticket = store.get_mut(id as usize).and_then(|t| t.take());
        ticket.ok_or(Error::TicketDeleteFailIdNotFound { id })
	}
}

// endregion: Ticket Controller