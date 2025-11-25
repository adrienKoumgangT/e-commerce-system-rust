create procedure app_user_get_by_country(IN __user_country varchar(20), IN __limit int, IN __offset int, IN __meta_user bigint)
begin

    if(__limit is not null and __offset is not null ) then

        select *
        from user_view
        where country = __user_country;

    else

        select *
        from user_view
        where country = __user_country;

    end if;

end;

