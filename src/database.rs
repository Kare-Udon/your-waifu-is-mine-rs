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

    pub fn get_twitter_users() -> Vec<String> {
        let options: Options = Options {
            create_if_missing: true,
            ..Options::default()
        };
        let mut db = DB::open("twitter_users", options).unwrap();
        let mut users = Vec::new();
        for mut value in db.new_iter() {
            let (_key, val) = value.next().unwrap();
            users.push(String::from_utf8(val).unwrap().to_string());
        }
        users
    }

    pub fn get_pixiv_users() -> Vec<String> {
        let options: Options = Options {
            create_if_missing: true,
            ..Options::default()
        };
        let mut db = DB::open("pixiv_users", options).unwrap();
        let mut users = Vec::new();
        for mut value in db.new_iter() {
            let (_key, val) = value.next().unwrap();
            users.push(String::from_utf8(val).unwrap().to_string());
        }
        users
    }
}
