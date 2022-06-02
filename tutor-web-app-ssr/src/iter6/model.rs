use actix_web::web;
use serde::{Deserialize, Serialize};

// TUTOR MODEL
#[derive(Serialize, Deserialize, Debug)]
pub struct TutorRegisterForm {
    pub username: String,
    pub password: String,
    pub confirmation: String,
    pub name: String,
    pub imageurl: String,
    pub profile: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TutorResponse {
    pub tutor_id: i32,
    pub tutor_name: String,
    pub tutor_pic_url: String,
    pub tutor_profile: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub username: String,
    pub tutor_id: i32,
    pub user_password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TutorSigninForm {
    pub username: String,
    pub password: String,
}
// ----

// NEW COURSES MODEL
#[derive(Serialize, Deserialize, Debug)]
pub struct NewCourse {
    pub course_name: String,
    pub course_description: String,
    pub course_format: String,
    pub course_duration: String,
    pub course_structure: Option<String>,
    pub course_price: Option<i32>,
    pub course_lenguage: Option<String>,
    pub course_level: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewCourseResponse {
    pub course_id: i32,
    pub tutor_id: i32,
    pub course_name: String,
    pub course_description: String,
    pub course_format: String,
    pub course_duration: String,
    pub course_structure: Option<String>,
    pub course_price: Option<i32>,
    pub course_lenguage: Option<String>,
    pub course_level: Option<String>,
    pub posted_time: String,
}

impl From<web::Json<NewCourseResponse>> for NewCourseResponse {
    fn from(new_course: web::Json<NewCourseResponse>) -> Self {
        Self {
            course_id: new_course.course_id,
            tutor_id: new_course.tutor_id,
            course_name: new_course.course_name.clone(),
            course_description: new_course.course_description.clone(),
            course_format: new_course.course_format.clone(),
            course_duration: new_course.course_duration.clone(),
            course_structure: new_course.course_structure.clone(),
            course_price: new_course.course_price,
            course_lenguage: new_course.course_lenguage.clone(),
            course_level: new_course.course_level.clone(),
            posted_time: new_course.posted_time.clone(),
        }
    }
}
// ---

// UPDATE COURSE MODEL
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateCourse {
    pub course_name: Option<String>,
    pub course_description: Option<String>,
    pub course_format: Option<String>,
    pub course_duration: Option<String>,
    pub course_structure: Option<String>,
    pub course_price: Option<i32>,
    pub course_lenguage: Option<String>,
    pub course_level: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateCourseResponse {
    pub course_id: i32,
    pub tutor_id: i32,
    pub course_name: String,
    pub course_description: String,
    pub course_format: String,
    pub course_duration: String,
    pub course_structure: String,
    pub course_price: i32,
    pub course_lenguage: String,
    pub course_level: String,
    pub posted_time: String,
}

impl From<web::Json<UpdateCourseResponse>> for UpdateCourseResponse {
    fn from(new_course: web::Json<UpdateCourseResponse>) -> Self {
        Self {
            course_id: new_course.course_id,
            tutor_id: new_course.tutor_id,
            course_name: new_course.course_name.clone(),
            course_description: new_course.course_description.clone(),
            course_format: new_course.course_format.clone(),
            course_duration: new_course.course_duration.clone(),
            course_structure: new_course.course_structure.clone(),
            course_price: new_course.course_price,
            course_lenguage: new_course.course_lenguage.clone(),
            course_level: new_course.course_level.clone(),
            posted_time: new_course.posted_time.clone(),
        }
    }
}
// ----
