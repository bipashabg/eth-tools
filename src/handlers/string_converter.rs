use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct StringRequest {
    input_string: String,
    conversion_type: String, 
}

#[derive(Serialize)]
pub struct StringResponse {
    output_string: String,
}

pub async fn convert_string(req: web::Json<StringRequest>) -> HttpResponse {
    let converted_string = match req.conversion_type.as_str() {
        "uppercase" => req.input_string.to_uppercase(),
        "lowercase" => req.input_string.to_lowercase(),
        _ => return HttpResponse::BadRequest().body("Invalid conversion type"),
    };

    HttpResponse::Ok().json(StringResponse {
        output_string: converted_string,
    })
}

