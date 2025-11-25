create procedure app_user_get_by_auth(IN __user_auth bigint, IN __limit int, IN __offset int, IN __meta_user bigint)
begin

    if(__limit is not null and __offset is not null ) then

        select *
        from user_view
        where auth = __user_auth
        limit __offset, __limit;

    else

        select *
        from user_view
        where auth = __user_auth;

    end if;

end;

