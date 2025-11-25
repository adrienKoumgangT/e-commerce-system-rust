-- auto-generated definition
create table if not exists user
(
    id              bigint auto_increment
        primary key,
    first_name      varchar(200)                       not null,
    last_name       varchar(300)                       not null,
    username        varchar(300)                       not null,
    password        varchar(500)                       null,
    profile_pic_url longtext                           null,
    auth            bigint                             null,
    status          bigint                             null,
    hired_date      datetime                           null,
    title           varchar(200)                       null,
    address         varchar(500)                       null,
    country         varchar(20)                        null,
    phone           varchar(20)                        null,
    created_at      datetime default CURRENT_TIMESTAMP not null,
    updated_at      datetime default CURRENT_TIMESTAMP not null,
    constraint user_pk_2
        unique (username),
    constraint user_user_auth_id_fk
        foreign key (auth) references user_auth (id),
    constraint user_user_status_id_fk
        foreign key (status) references user_status (id)
);

create index user_created_at_index
    on user (created_at);

create index user_hired_date_index
    on user (hired_date);

create index user_title_index
    on user (title);

create index user_username_index
    on user (username);

