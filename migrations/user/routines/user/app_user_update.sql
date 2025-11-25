create procedure app_user_update(IN __user_id bigint,
                                 IN __user_first_name varchar(200),
                                 IN __user_last_name varchar(300),
                                 IN __user_hired_date datetime,
                                 IN __user_title varchar(200),
                                 IN __user_address varchar(500),
                                 IN __user_country varchar(20),
                                 IN __user_phone varchar(20),
                                 IN __meta_user bigint)
begin

    update user
    set first_name = __user_first_name,
        last_name = __user_last_name,
        hired_date = __user_hired_date,
        title = __user_title,
        address = __user_address,
        country = __user_country,
        phone = __user_phone,
        updated_at = now()
    where id = __user_id;


    call app_user_get(__user_id, __meta_user);

end;

