create procedure app_user_auth_update(IN __user_auth_id bigint,
                                      IN __user_auth_name varchar(200),
                                      IN __user_auth_description longtext,
                                      IN __meta_user bigint)
begin

    update user_auth
    set name = __user_auth_name,
        description = __user_auth_description
    where id = __user_auth_id;

    call app_user_auth_get(last_insert_id(), __meta_user);

end;

