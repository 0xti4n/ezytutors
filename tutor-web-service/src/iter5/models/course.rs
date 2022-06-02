use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Course {
    pub course_id: i32,
    pub tutor_id: i32,
    pub course_name: String,
    pub course_description: Option<String>,
    pub course_format: Option<String>,
    pub course_structure: Option<String>,
    pub course_duration: Option<String>,
    pub course_price: Option<i32>,
    pub course_lenguage: Option<String>,
    pub course_level: Option<String>,
    pub posted_time: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateCourse {
    pub tutor_id: i32,
    pub course_name: String,
    pub course_description: Option<String>,
    pub course_format: Option<String>,
    pub course_structure: Option<String>,
    pub course_duration: Option<String>,
    pub course_price: Option<i32>,
    pub course_lenguage: Option<String>,
    pub course_level: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateCourse {
    pub course_name: Option<String>,
    pub course_description: Option<String>,
    pub course_format: Option<String>,
    pub course_structure: Option<String>,
    pub course_duration: Option<String>,
    pub course_price: Option<i32>,
    pub course_lenguage: Option<String>,
    pub course_level: Option<String>,
}

impl From<web::Json<CreateCourse>> for CreateCourse {
    fn from(new_course: web::Json<CreateCourse>) -> Self {
        Self {
            tutor_id: new_course.tutor_id,
            course_name: new_course.course_name.clone(),
            course_description: new_course.course_description.clone(),
            course_format: new_course.course_format.clone(),
            course_structure: new_course.course_structure.clone(),
            course_level: new_course.course_level.clone(),
            course_duration: new_course.course_duration.clone(),
            course_lenguage: new_course.course_lenguage.clone(),
            course_price: new_course.course_price,
        }
    }
}

impl From<web::Json<UpdateCourse>> for UpdateCourse {
    fn from(new_course: web::Json<UpdateCourse>) -> Self {
        Self {
            course_name: new_course.course_name.clone(),
            course_description: new_course.course_description.clone(),
            course_format: new_course.course_format.clone(),
            course_structure: new_course.course_structure.clone(),
            course_level: new_course.course_level.clone(),
            course_duration: new_course.course_duration.clone(),
            course_lenguage: new_course.course_lenguage.clone(),
            course_price: new_course.course_price,
        }
    }
}
