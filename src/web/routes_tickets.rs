use crate::ctx::Ctx;
use crate::model::{ModelController, Ticket, TicketForCreate};
use crate::Result;
use axum::extract::{Path, State};
use axum::routing::{delete, post};
use axum::{Json, Router};


pub fn routes(mc: ModelController) -> Router {
	Router::new()
		.route("/tickets", post(create_ticket).get(list_tickets))
		.route("/tickets/:id", delete(delete_ticket))
		.with_state(mc)
}

async fn create_ticket(
	State(mc): State<ModelController>,
    ctx: Ctx,
	Json(ticket_fc): Json<TicketForCreate>,
) -> Result<Json<Ticket>> {
	println!("->> {:<12} - create_ticket", "HANDLER");

	let ticket = mc.create_ticket(ticket_fc, ctx).await?;

	Ok(Json(ticket))
}

async fn list_tickets(
	State(mc): State<ModelController>,
    ctx: Ctx,
) -> Result<Json<Vec<Ticket>>> {
	println!("->> {:<12} - list_tickets", "HANDLER");

	let tickets = mc.list_tickets(ctx).await?;

	Ok(Json(tickets))
}

async fn delete_ticket(
	State(mc): State<ModelController>,
    ctx: Ctx,
	Path(id): Path<u64>,
) -> Result<Json<Ticket>> {
	println!(">>> {:<12} - delete_ticket", "HANDLER");

	let ticket = mc.delete_ticket(id, ctx).await?;

	Ok(Json(ticket))
}