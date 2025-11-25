create procedure app_user_search_by_country(IN __user_country varchar(20), IN __limit int, IN __offset int, IN __meta_user bigint)
begin

    if(__limit is not null and __offset is not null ) then

        select *
        from user_view
        where country like concat(__user_country, '%')
        limit __offset, __limit;

    else

        select *
        from user_view
        where country like concat(__user_country, '%');

    end if;

end;

