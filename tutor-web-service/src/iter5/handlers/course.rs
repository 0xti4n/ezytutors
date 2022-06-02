use crate::dbaccess::course::*;
use crate::errors::EzyTutorError;
use crate::models::course::{CreateCourse, UpdateCourse};
use crate::state::AppState;
use actix_web::{web, HttpResponse};
use std::convert::TryFrom;

pub async fn get_courses_for_tutor(
    app_state: web::Data<AppState>,
    params: web::Path<(i32,)>,
) -> Result<HttpResponse, EzyTutorError> {
    get_courses_for_tutor_db(&app_state.db, params.0)
        .await
        .map(|courses| HttpResponse::Ok().json(courses))
}

pub async fn get_course_details(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, EzyTutorError> {
    get_course_details_db(&app_state.db, params.0, params.1)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn post_new_course(
    app_state: web::Data<AppState>,
    new_course: web::Json<CreateCourse>,
) -> Result<HttpResponse, EzyTutorError> {
    post_new_course_db(&app_state.db, new_course.into())
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn delete_course(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, EzyTutorError> {
    delete_course_db(&app_state.db, params.0, params.1)
        .await
        .map(|resp| HttpResponse::Ok().json(resp))
}

pub async fn update_course_details(
    app_state: web::Data<AppState>,
    update_course: web::Json<UpdateCourse>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, EzyTutorError> {
    update_course_details_db(&app_state.db, params.0, params.1, update_course.into())
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

#[cfg(test)]
mod test {
    use super::*;
    use actix_web::http::StatusCode;
    use actix_web::ResponseError;
    //use chrono::NaiveDate;
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

        let tutor_id: web::Path<(i32,)> = web::Path::from((1,));
        let res = get_courses_for_tutor(app_state, tutor_id).await.unwrap();

        assert_eq!(res.status(), StatusCode::OK)
    }

    #[actix_rt::test]
    async fn get_course_detail_success_test() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let db_pool = PgPool::connect(&database_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let params: web::Path<(i32, i32)> = web::Path::from((1, 2));
        let res = get_course_details(app_state, params).await.unwrap();

        assert_eq!(res.status(), StatusCode::OK)
    }

    #[actix_rt::test]
    async fn get_course_detail_failure_test() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let db_pool = PgPool::connect(&database_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let params: web::Path<(i32, i32)> = web::Path::from((1, 21));
        let res = get_course_details(app_state, params).await;

        match res {
            Ok(_) => println!("Something wrong"),
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND),
        }
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

        let new_course_msg = CreateCourse {
            tutor_id: 1,
            course_name: "This is the next course".into(),
            course_description: Some("This is a test course".into()),
            course_format: None,
            course_level: Some("Beginner".into()),
            course_price: None,
            course_duration: None,
            course_lenguage: Some("English".into()),
            course_structure: None,
        };

        let res = post_new_course(app_state, web::Json(new_course_msg))
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::OK)
    }

    #[actix_rt::test]
    async fn update_course_success() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let db_pool = PgPool::connect(&database_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let new_course_msg = UpdateCourse {
            course_name: Some("Course name changed".into()),
            course_description: Some("This is a yet another test course".into()),
            course_format: None,
            course_level: Some("Intermediate".into()),
            course_price: None,
            course_duration: None,
            course_lenguage: Some("German".into()),
            course_structure: None,
        };

        let parameters = web::Path::from((1, 2));
        let res = update_course_details(app_state, web::Json(new_course_msg), parameters)
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::OK)
    }

    #[actix_rt::test]
    async fn delete_test_success() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let db_pool = PgPool::connect(&database_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let parameters = web::Path::from((1, 5));
        let res = delete_course(app_state, parameters).await.unwrap();

        assert_eq!(res.status(), StatusCode::OK)
    }

    #[actix_rt::test]
    async fn delete_test_failure() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let db_pool = PgPool::connect(&database_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let parameters = web::Path::from((1, 21));
        let res = delete_course(app_state, parameters).await;

        match res {
            Ok(_) => println!("Something wrong"),
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND),
        }
    }
}
