DROP TABLE IF EXISTS Schedule; 
DROP TABLE IF EXISTS Users; 
DROP TABLE IF EXISTS Events; 
 
 
CREATE TABLE Users ( 
    username VARCHAR PRIMARY KEY, 
    password VARCHAR NOT NULL 
); 
 
CREATE TABLE Events ( 
    id SERIAL PRIMARY KEY, 
    title VARCHAR NOT NULL, 
    date TIMESTAMP WITH TIME ZONE NOT NULL, 
    duration INTERVAL NOT NULL, 
    creation_date TIMESTAMP WITH TIME ZONE NOT NULL, 
    description VARCHAR, 
    CONSTRAINT future_event CHECK ( date > creation_date ) 
); 
 
CREATE TABLE Schedule ( 
    username VARCHAR REFERENCES Users(username), 
    event SERIAL REFERENCES Events(id) 
);