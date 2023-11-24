use actix_web::{HttpRequest, HttpResponse};
use chrono::{FixedOffset, Local, TimeZone, Utc};
use serde_json::json;

use crate::routes::error::Error;
use actix_web::Result;

use crate::CronManager::CronManager;

fn test() {
    println!("now: {}", Local::now().to_string());
}

pub async fn start_cron(req: HttpRequest, manager: &'static CronManager) -> Result<HttpResponse> {
    let utc_tz = Utc;
    let mut cron = cron_tab::Cron::new(utc_tz);
    let job_test_id = cron.add_fn("* * * * * * *", test).unwrap();
    cron.start();

    Ok(HttpResponse::Ok().json({
        json!({
            "status": "ok",
            "message": "test cron api",
            "id": job_test_id,
        })
    }))
}

pub async fn stop_cron(req: HttpRequest, manager: &CronManager) -> Result<HttpResponse> {
    let cron = cron_tab::Cron::new(Utc);
    // let entries = cron.entries.lock().unwrap();
    Ok(HttpResponse::Ok().json({
        json!({
            "status": "ok",
            "message": "test cron api",
        })
    }))
}
