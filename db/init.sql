CREATE TABLE userInfo(
    id SERIAL PRIMARY KEY,
    name varchar(20) NOT NULL,
    password varchar(20) NOT NULL
);
INSERT INTO userinfo(name, password) VALUES(
    'admin',
    'secret'
);