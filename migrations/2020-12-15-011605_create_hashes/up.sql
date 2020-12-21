create table hashes
(
    sha256 char(64) primary key,
    md5    char(32) not null,
    index  md5_index(md5)
)