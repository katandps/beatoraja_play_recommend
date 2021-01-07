create table users
(
    id int primary key auto_increment,
    google_id varchar(64) not null,
    gmail_address varchar(255) not null,
    name varchar(255) not null,
    registered_date datetime not null,
    unique key google_id_unique_key (google_id),
    unique key gmail_address (gmail_address)
);