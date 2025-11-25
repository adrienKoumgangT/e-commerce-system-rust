create procedure app_user_auth_get_by_name(IN __user_auth_name varchar(200), IN __meta_user bigint)
begin

    select *
    from user_auth
    where name = __user_auth_name;

end;

