-- Create `userInfo` Table
CREATE TABLE userInfo(
    id SERIAL PRIMARY KEY,
    name varchar(20) NOT NULL,
    password varchar(20) NOT NULL
);