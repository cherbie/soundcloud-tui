// use reqwest;

pub struct UserInfo<'a> {
    id: &'a str,
    full_name: &'a str,
    last_name: &'a str,
    username: &'a str,
}

pub fn get_user_info<'a, 'b>(token: &'b str) -> UserInfo<'a> {
    return UserInfo {
        id: "id",
        full_name: "sdf",
        last_name: "df",
        username: "sdfd",
    };
}

#[cfg(test)]
mod test {
    use super::get_user_info;

    #[test]
    fn test_get_user_info() {
        let user_info = get_user_info("asdfdsf");
        assert_eq!(user_info.id, "id");
        assert_eq!(user_info.full_name, "sdf");
        assert_eq!(user_info.last_name, "df");
        assert_eq!(user_info.username, "sdfd");
    }
}
