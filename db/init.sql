CREATE TABLE userInfo(
    id SERIAL PRIMARY KEY,
    name varchar(20) NOT NULL,
    password varchar(20) NOT NULL
);
INSERT INTO userinfo(name, password) VALUES(
    'admin',
    'secret'
);

CREATE TABLE event(
    id SERIAL PRIMARY KEY,
    user_id INTEGER,
    note TEXT NOT NULL,
    event_date DATE NOT NULL,
    CONSTRAINT fk_user_id FOREIGN KEY(user_id) REFERENCES userInfo(id) ON DELETE CASCADE ON UPDATE CASCADE
);