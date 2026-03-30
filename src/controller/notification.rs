use rocket::response::status::Created;
use rocket::serde::json::Json;

use bambangshop::Result;
use crate::model::subscriber::Subscriber;
use crate::service::notification::NotificationService;

#[post("/subscribe/<product_type>", data = "<subscriber>")]
pub fn subscribe(product_type: &str, subscriber: Subscriber) -> Result<Created<Json<Subscriber>>> {
    return match NotificationService::subscribe(product_type, subscriber) {
        Ok(subscriber_result) => Ok(Created::new("/notification/subscribe").body(Json(subscriber_result))),
        Err(e) => Err(e),
    };
}