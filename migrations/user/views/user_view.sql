create view user_view as
select
    user.id,
    first_name,
    last_name,
    username,
    password,
    profile_pic_url,
    auth,
    user_auth.name as auth_name,
    user_auth.description as auth_description,
    status,
    user_status.name as status_name,
    user_status.description as status_description,
    hired_date,
    title,
    address,
    country,
    phone,
    created_at,
    updated_at
from user
    left join user_auth on user.auth = user_auth.id
    left join user_status on user.status = user_status.id
;
