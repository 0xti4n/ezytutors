use crate::iter6::state::AppState;
use crate::model::{NewCourse, NewCourseResponse, UpdateCourse, UpdateCourseResponse};
use actix_web::{web, Error, HttpResponse, Result};
use serde_json::json;

pub async fn handle_insert_course(
    _tmpl: web::Data<tera::Tera>,
    _app_state: web::Data<AppState>,
    tutor_id: web::Path<i32>,
    params: web::Json<NewCourse>,
) -> Result<HttpResponse, Error> {
    let new_course = json!({
        "tutor_id": *tutor_id,
        "course_name": &params.course_name,
        "course_description": &params.course_description,
        "course_format": &params.course_format,
        "course_duration": &params.course_duration,
        "course_structure": &params.course_structure,
        "course_price": &params.course_price,
        "course_lenguage": &params.course_lenguage,
        "course_level": &params.course_level,
    });

    let awc_client = awc::Client::default();
    let res = awc_client
        .post("http://localhost:3000/courses/")
        .send_json(&new_course)
        .await
        .unwrap()
        .body()
        .await?;

    println!("Finished Call: {:?}", res);
    let course_response: NewCourseResponse = serde_json::from_str(&std::str::from_utf8(&res)?)?;

    Ok(HttpResponse::Ok().json(course_response))
}

pub async fn handle_update_course(
    _tmpl: web::Data<tera::Tera>,
    _app_state: web::Data<AppState>,
    params_tutor_info: web::Path<(i32, i32)>,
    params: web::Json<UpdateCourse>,
) -> Result<HttpResponse, Error> {
    let update_course = json!({
        "course_name": &params.course_name,
        "course_description": &params.course_description,
        "course_format": &params.course_format,
        "course_duration": &params.course_duration,
        "course_structure": &params.course_structure,
        "course_price": &params.course_price,
        "course_lenguage": &params.course_lenguage,
        "course_level": &params.course_level,
    });

    let awc_client = awc::Client::default();
    let update_url = format!(
        "http://localhost:3000/courses/{}/{}",
        params_tutor_info.0, params_tutor_info.1
    );
    let res = awc_client
        .put(update_url)
        .send_json(&update_course)
        .await
        .unwrap()
        .body()
        .await?;

    let course_response: UpdateCourseResponse = serde_json::from_str(&std::str::from_utf8(&res)?)?;
    Ok(HttpResponse::Ok().json(course_response))
}

pub async fn handle_delete_course(
    _tmpl: web::Data<tera::Tera>,
    _app_state: web::Data<AppState>,
    params_tutor_info: web::Path<(i32, i32)>,
) -> Result<HttpResponse, Error> {
    let tutor_id = params_tutor_info.0;
    let course_id = params_tutor_info.1;
    let awc_client = awc::Client::default();
    let delete_url = format!("http://localhost:3000/courses/{}/{}", tutor_id, course_id);

    let _res = awc_client.delete(delete_url).send().await.unwrap();

    Ok(HttpResponse::Ok().body("Course deleted"))
}
