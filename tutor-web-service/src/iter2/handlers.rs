use super::models::Course;
use super::state::AppState;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse};
use sqlx::postgres::PgPool;

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;

    HttpResponse::Ok().json(&response)
}

pub async fn get_courses_for_tutor(
    app_state: web::Data<AppState>,
    params: web::Path<(usize,)>,
) -> HttpResponse {
    HttpResponse::Ok().json("Success")
}

pub async fn get_course_details(
    app_state: web::Data<AppState>,
    params: web::Path<(usize, usize)>,
) -> HttpResponse {
    HttpResponse::Ok().json("Success")
}

pub async fn post_new_course(
    app_state: web::Data<AppState>,
    new_course: web::Json<Course>,
) -> HttpResponse {
    HttpResponse::Ok().json("Success")
}

#[cfg(test)]
mod test {
    use super::*;
    use actix_web::http::StatusCode;
    use chrono::NaiveDate;
    use dotenv::dotenv;
    use sqlx::postgres::PgPool;
    use std::env;
    use std::sync::Mutex;

    #[actix_rt::test]
    async fn get_all_courses_success() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let db_pool = PgPool::connect(&database_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let tutor_id: web::Path<(usize,)> = web::Path::from((1,));
        let res = get_courses_for_tutor(app_state, tutor_id).await;

        assert_eq!(res.status(), StatusCode::OK)
    }

    #[actix_rt::test]
    async fn get_course_detail_test() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let db_pool = PgPool::connect(&database_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let params: web::Path<(usize, usize)> = web::Path::from((1, 2));
        let res = get_course_details(app_state, params).await;

        assert_eq!(res.status(), StatusCode::OK)
    }

    #[actix_rt::test]
    async fn post_course_success() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let db_pool = PgPool::connect(&database_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let new_course_msg = Course {
            course_id: 1,
            tutor_id: 1,
            course_name: "This is the next course".into(),
            posted_time: Some(NaiveDate::from_ymd(2022, 5, 6).and_hms(14, 49, 30)),
        };

        let res = post_new_course(app_state, web::Json(new_course_msg)).await;

        assert_eq!(res.status(), StatusCode::OK)
    }
}
