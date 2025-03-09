use std::net::IpAddr;
use std::sync::Arc;
use actix_web::{web, HttpRequest, HttpResponse};
use maxminddb::{geoip2, Reader};
use crate::AppState;
use crate::models::{SimpleResponse};
use crate::util::{format_response, get_ip, QueryOptions};

pub async fn region_handler(
    req: HttpRequest,
    state: web::Data<Arc<AppState>>,
    query: web::Query<QueryOptions>,
) -> HttpResponse {
    format_response(query.format.as_deref(), &get_region_response(&req, &state))
}

pub fn get_region(ip: IpAddr, geo_db: &Reader<Vec<u8>>) -> Option<String> {
    geo_db.lookup::<geoip2::City>(ip).ok()?
        .subdivisions.as_ref()?
        .first()?
        .names.as_ref()?
        .get("en")
        .cloned()
        .map(String::from)
}
pub fn get_region_response(req: &HttpRequest, state: &web::Data<Arc<AppState>>) -> SimpleResponse {
    let result = get_region(get_ip(&req), &state.geo_db).unwrap_or_else(|| "".to_string());
    SimpleResponse { value: result }
}