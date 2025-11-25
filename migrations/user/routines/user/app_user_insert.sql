create procedure app_user_insert(IN __user_first_name varchar(200),
                                 IN __user_last_name varchar(300),
                                 IN __user_username varchar(300),
                                 IN __user_password varchar(500),
                                 IN __user_profile_pic_url longtext,
                                 IN __user_auth bigint,
                                 IN __user_status bigint,
                                 IN __user_hired_date datetime,
                                 IN __user_title varchar(200),
                                 IN __user_address varchar(500),
                                 IN __user_country varchar(20),
                                 IN __user_phone varchar(20),
                                 IN __meta_user bigint)
begin

    insert into user
        (
            first_name,
            last_name,
            username,
            password,
            profile_pic_url,
            auth,
            status,
            hired_date,
            title,
            address,
            country,
            phone,
            created_at,
            updated_at
        )
    values
        (
            __user_first_name,
            __user_last_name,
            __user_username,
            __user_password,
            __user_profile_pic_url,
            __user_auth,
            __user_status,
            __user_hired_date,
            __user_title,
            __user_address,
            __user_country,
            __user_phone,
            now(),
            now()
        )
    ;

    call app_user_get(last_insert_id(), __meta_user);

end;

