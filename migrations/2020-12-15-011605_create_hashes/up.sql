create table hashes
(
    sha256 varchar(64) primary key,
    md5 varchar(32) not null,
    index md5_index(md5)
)