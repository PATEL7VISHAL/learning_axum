use crate::ctx::Ctx;
use crate::error::{Error, Result};
use crate::model::{ModelController, Ticket, TicketForCreate};
use axum::extract::{FromRef, Path};
use axum::routing::{delete, post};
use axum::Router;
use axum::{extract::State, Json};

// here is the parent state
#[derive(Clone, FromRef)]
pub struct AppState {
    mc: ModelController, // and ModelController is substate
}

pub fn routes(mc: ModelController) -> Router {
    let app_state = AppState { mc };
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/:id", delete(delete_ticket))
        // .with_state(mc) // this one also work fine
        .with_state(app_state) // though it need ModelController for below theire routes but because
                               // of parent state is derived from `FromRef` it working fine on using
                               // the paretal state
}

// region:  -- REST Handlers
async fn create_ticket(
    State(mc): State<ModelController>, // also the `State` impl `deref` trait so not need get
    ctx: Ctx,
    // ModelController vai destructuring
    Json(ticket_fc): Json<TicketForCreate>,
) -> Result<Json<Ticket>> {
    println!("->> {:<25} - create_ticket", "HANDLER");
    let ticket = mc.create_ticket(ctx, ticket_fc).await?;
    Ok(Json(ticket))
}

async fn list_tickets(mc: State<ModelController>, ctx: Ctx) -> Result<Json<Vec<Ticket>>> {
    println!("->> {:<25} - list_tickets", "HANDLER");
    let tickets = mc.list_tickets(ctx).await?;
    println!("tickets: {:#?}", tickets);
    Ok(Json(tickets))
}

async fn delete_ticket(
    mc: State<ModelController>,
    ctx: Ctx,
    Path(id): Path<u64>,
) -> Result<Json<Ticket>> {
    println!("->> {:<25} - delete_ticket", "HANDLER");
    let ticket = mc.delete_ticket(ctx, id).await?;
    Ok(Json(ticket))
}
// endregion:  -- REST Handlers
