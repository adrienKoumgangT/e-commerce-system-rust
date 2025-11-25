create procedure app_user_auth_get_all(IN __meta_user bigint)
begin

    select *
    from user_auth;

end;

