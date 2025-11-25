create procedure app_user_search_by_username(IN __user_username varchar(300), IN __limit int, IN __offset int, IN __meta_user bigint)
begin

    if(__limit is not null and __offset is not null ) then

        select *
        from user_view
        where username like concat(__user_username, '%')
        limit __offset, __limit;

    else

        select *
        from user_view
        where username like concat(__user_username, '%');

    end if;

end;

