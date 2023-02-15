-- Create `event` Table
CREATE TABLE event(
    id SERIAL PRIMARY KEY,
    user_id INTEGER,
    note TEXT NOT NULL,
    event_date DATE NOT NULL,
    CONSTRAINT fk_user_id FOREIGN KEY(user_id) REFERENCES userInfo(id) ON DELETE CASCADE ON UPDATE CASCADE
);