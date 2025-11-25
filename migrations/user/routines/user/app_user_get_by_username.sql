create procedure app_user_get_by_username(IN __user_username varchar(300), IN __meta_user bigint)
begin

    select *
    from user_view
    where username = __user_username;

end;

