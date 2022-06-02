DROP TABLE IF EXISTS ezy_course_c6 CASCADE;
DROP TABLE IF EXISTS ezy_tutor_c6;

CREATE TABLE ezy_tutor_c6 (
    tutor_id SERIAL PRIMARY KEY,
    tutor_name VARCHAR(200) NOT NULL,
    tutor_pic_url VARCHAR(200) NOT NULL,
    tutor_profile VARCHAR(200) NOT NULL
);

CREATE TABLE ezy_course_c6 (
    course_id SERIAL PRIMARY KEY,
    tutor_id INT NOT NULL,
    course_name VARCHAR(140) NOT NULL,
    course_description VARCHAR(2000),
    course_format VARCHAR(30),
    course_structure VARCHAR(200),
    course_duration VARCHAR(30),
    course_price INT,
    course_lenguage VARCHAR(30),
    course_level VARCHAR(30),
    posted_time TIMESTAMP DEFAULT NOW(),
    CONSTRAINT fk_tutor
    FOREIGN KEY(tutor_id)
    REFERENCES ezy_tutor_c6(tutor_id)
);

GRANT ALL PRIVILEGES ON TABLE ezy_tutor_c6 TO xtian;
GRANT ALL PRIVILEGES ON TABLE ezy_course_c6 TO xtian;

/* load data for testing */
/* INSERT INTO ezy_tutor_c6(tutor_id, tutor_name, tutor_pic_url, tutor_profile)
VALUES (1, 'Merlene', 'http://s3.amazon.aws.com/pic1', 'Merlene is an experienced finance professional');

INSERT INTO ezy_tutor_c6(tutor_id, tutor_name, tutor_pic_url, tutor_profile)
VALUES (2, 'Frank', 'http://s3.amazon.aws.com/pic2', 'Frank is an expert nuclear engineer');

INSERT INTO ezy_course_c6(course_id, tutor_id, course_name, course_level, posted_time)
VALUES (1, 1, 'First course', 'Beginner', '2022-05-31 05:40:00');

INSERT INTO ezy_course_c6(course_id, tutor_id, course_name, course_format, posted_time)
VALUES (2, 1, 'Second course', 'ebook', '2022-05-31 05:45:00'); */
