create table users
(
    id int primary key auto_increment,
    gmail_address varchar(255) not null,
    name varchar(255) not null,
    registered_date datetime not null
);