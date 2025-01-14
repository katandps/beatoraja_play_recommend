create table revoked_sessions
(
    id int primary key auto_increment,
    session_key text not null,
    user_id int not null,
    revoked_at datetime not null,
    foreign key revoked_sessions (user_id) references users (id)
)
