DROP TABLE IF EXISTS ezy_course_c4;

CREATE TABLE ezy_course_c4 (
    course_id serial primary key,
    tutor_id INT NOT NULL,
    course_name VARCHAR(140) NOT NULL,
    posted_time TIMESTAMP DEFAULT NOW()
);

INSERT INTO ezy_course_c4(course_id, tutor_id, course_name, posted_time) VALUES(1, 1, 'Firts course', '2022-05-06 11:31:00');
INSERT INTO ezy_course_c4(course_id, tutor_id, course_name, posted_time) VALUES(2, 1, 'Second course', '2022-05-06 11:35:00');
