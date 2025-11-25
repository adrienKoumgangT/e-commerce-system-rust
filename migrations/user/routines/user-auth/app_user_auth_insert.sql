create procedure app_user_auth_insert(IN __user_auth_name varchar(200),
                                      IN __user_auth_description longtext,
                                      IN __meta_user bigint)
begin

    insert into user_auth
    (
        name,
        description
    )
    VALUES
        (
            __user_auth_name,
            __user_auth_description
        )
    ;

    call app_user_auth_get(last_insert_id(), __meta_user);

end;

