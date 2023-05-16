//! Simplistic Model Layer
//! with mock-store layer

use crate::{ctx::Ctx, Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// region:    --- Ticket Types
#[derive(Debug, Clone, Serialize)]
pub struct Ticket {
  pub id: u64,
  pub ctx_id: u64,
  pub title: String,
}

#[derive(Deserialize)]
pub struct TicketForCreate {
  pub title: String,
}
// endregion: --- Ticket Types

// region:    --- Model Controller
#[derive(Clone)]
pub struct ModelController {
  tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

impl ModelController {
  pub async fn new() -> Result<Self> {
    Ok(Self {
      tickets_store: Arc::default(),
    })
  }

  pub async fn create_ticket(&self, ctx: Ctx, ticket_fc: TicketForCreate) -> Result<Ticket> {
    let mut store = self.tickets_store.lock().unwrap();

    let id = store.len() as u64;
    let title = ticket_fc.title;
    let ctx_id = ctx.user_id();
    let ticket = Ticket { id, ctx_id, title };

    store.push(Some(ticket.clone()));

    Ok(ticket)
  }

  pub async fn list_tickets(&self, ctx: Ctx) -> Result<Vec<Ticket>> {
    let store = self.tickets_store.lock().unwrap();

    let tickets = store.iter().filter_map(|t| t.clone()).collect();
    Ok(tickets)
  }

  pub async fn delete_ticket(&self, ctx: Ctx, id: u64) -> Result<Ticket> {
    let mut store = self.tickets_store.lock().unwrap();

    let ticket = store.get_mut(id as usize).and_then(|t| t.take());

    ticket.ok_or(Error::TicketDeleteFailIdNotFound { id })
  }
}
