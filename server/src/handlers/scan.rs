use actix_web::{web, HttpResponse, Responder};
use crate::{task::TaskMessage, AppState};


pub async fn scan_songs(state: web::Data<AppState>) -> Result<impl Responder, actix_web::Error> {
    let addr = &state.config.addr;
    addr.do_send(TaskMessage("Scan songs".to_string()));
    
    Ok(HttpResponse::Ok().body("Scan songs"))
}