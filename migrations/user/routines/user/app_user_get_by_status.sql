create procedure app_user_get_by_status(IN __user_status bigint, IN __limit int, IN __offset int, IN __meta_user bigint)
begin

    if(__limit is not null and __offset is not null ) then

        select *
        from user_view
        where status = __user_status
        limit __offset, __limit;

    else

        select *
        from user_view
        where status = __user_status;

    end if;

end;

