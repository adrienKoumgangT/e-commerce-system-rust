create procedure app_user_get_by_country_and_or_title(IN __user_country varchar(20),
                                                      IN __user_title varchar(200),
                                                      IN __limit int, IN __offset int,
                                                      IN __meta_user bigint)
begin

    if(__limit is not null and __offset is not null ) then
        select *
        from user_view
        where (__user_country is null or country = __user_country)
          and (__user_title is null or title = __user_title)
        limit __offset, __limit
        ;
    else
        select *
        from user_view
        where (__user_country is null or country = __user_country)
          and (__user_title is null or title = __user_title)
        ;
    end if;

end;
