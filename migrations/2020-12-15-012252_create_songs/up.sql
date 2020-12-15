create table songs
(
    sha256 varchar(64) not null primary key,
    title varchar(255) not null,
    subtitle varchar(255) not null,
    artist varchar(255) not null,
    sub_artist varchar(255) not null,
    notes int not null,
    length int not null,
    foreign key songs_hash(sha256) references hashes(sha256)
);

