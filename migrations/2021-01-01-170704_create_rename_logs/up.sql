create table rename_logs
(
    id int primary key auto_increment,
    user_id int not null,
    old_name varchar(255) not null,
    new_name varchar(255) not null,
    date datetime not null
)