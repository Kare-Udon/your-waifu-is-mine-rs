pub mod database {
    use rusty_leveldb::{DB, Options, DBIterator, LdbIterator};


    pub fn add_twitter_user(&user_id: &i64, user_name: &String) {
        let options: Options = Options {
            create_if_missing: true,
            ..Options::default()
        };
        let mut db = DB::open("twitter_users", options).unwrap();
        db.put(user_id.to_string().as_ref(), user_name.as_ref()).unwrap();
        db.flush().expect("TODO: panic message");
    }

    pub fn remove_twitter_user(&user_id: &i64) {
        let options: Options = Options {
            create_if_missing: true,
            ..Options::default()
        };
        let mut db = DB::open("twitter_users", options).unwrap();
        db.delete(user_id.to_string().as_ref()).unwrap();
        db.flush().expect("TODO: panic message");
    }

    pub fn add_pixiv_user(&user_id: &i64, user_name: &String) {
        let options: Options = Options {
            create_if_missing: true,
            ..Options::default()
        };
        let mut db = DB::open("pixiv_users", options).unwrap();
        db.put(user_id.to_string().as_ref(), user_name.as_ref()).unwrap();
        db.flush().expect("TODO: panic message");
    }

    pub fn remove_pixiv_user(&user_id: &i64) {
        let options: Options = Options {
            create_if_missing: true,
            ..Options::default()
        };
        let mut db = DB::open("pixiv_users", options).unwrap();
        db.delete(user_id.to_string().as_ref()).unwrap();
        db.flush().expect("TODO: panic message");
    }

    pub fn add_twitter_post(&post_id: &i64, post_url: &String) {
        let options: Options = Options {
            create_if_missing: true,
            ..Options::default()
        };
        let mut db = DB::open("twitter_posts", options).unwrap();
        db.put(post_id.to_string().as_ref(), post_url.as_ref()).unwrap();
        db.flush().expect("TODO: panic message");
    }

    pub fn add_pixiv_post(&post_id: &i64, post_url: &String) {
        let options: Options = Options {
            create_if_missing: true,
            ..Options::default()
        };
        let mut db = DB::open("pixiv_posts", options).unwrap();
        db.put(post_id.to_string().as_ref(), post_url.as_ref()).unwrap();
        db.flush().expect("TODO: panic message");
    }

    pub fn get_twitter_users() -> Vec<(String, String)> {
        let options: Options = Options {
            create_if_missing: true,
            ..Options::default()
        };
        let mut db = DB::open("twitter_users", options).unwrap();
        let mut users = Vec::new();
        let mut iter = db.new_iter().unwrap();
        while iter.advance() {
            let mut key: Vec<u8> = Vec::new();
            let mut value: Vec<u8>= Vec::new();
            if iter.current(&mut key, &mut value) {
                users.push((String::from_utf8(key).unwrap().to_string(), String::from_utf8(value).unwrap().to_string()));
            }
        }
        users
    }

    pub fn get_pixiv_users() -> Vec<(String, String)> {
        let options: Options = Options {
            create_if_missing: true,
            ..Options::default()
        };
        let mut db = DB::open("pixiv_users", options).unwrap();
        let mut users = Vec::new();
        let mut iter = db.new_iter().unwrap();
        while iter.advance() {
            let mut key: Vec<u8> = Vec::new();
            let mut value: Vec<u8>= Vec::new();
            if iter.current(&mut key, &mut value) {
                users.push((String::from_utf8(key).unwrap().to_string(), String::from_utf8(value).unwrap().to_string()));
            }
        }
        users
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn add_del_twitter_user_test() {
            use super::*;
            add_twitter_user(&324391547, &"avogado6".to_string());
            let users = get_twitter_users();
            assert_eq!(users[0], ("324391547".to_string(), "avogado6".to_string()));
            remove_twitter_user(&324391547);
            let users = get_twitter_users();
            assert_eq!(users.len(), 0);
        }

        #[test]
        fn add_del_pixiv_user_test() {
            use super::*;
            add_pixiv_user(&101484832, &"Nakkar".to_string());
            let users = get_pixiv_users();
            assert_eq!(users[0], ("101484832".to_string(), "Nakkar".to_string()));
            remove_pixiv_user(&101484832);
            let users = get_pixiv_users();
            assert_eq!(users.len(), 0);
        }
    }
}
