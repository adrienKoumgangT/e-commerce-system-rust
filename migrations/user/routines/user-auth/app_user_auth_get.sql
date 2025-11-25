create procedure app_user_auth_get(IN __user_auth_id bigint, IN __meta_user bigint)
begin

    select *
    from user_auth
    where id = __user_auth_id;

end;

