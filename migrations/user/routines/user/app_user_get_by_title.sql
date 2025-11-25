create procedure app_user_get_by_title(IN __user_title varchar(200), IN __limit int, IN __offset int, IN __meta_user bigint)
begin

    if(__limit is not null and __offset is not null ) then

        select *
        from user_view
        where title = __user_title
        limit __offset, __limit;

    else

        select *
        from user_view
        where title = __user_title;

    end if;



end;

