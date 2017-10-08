extern crate shared_structs;

struct AppState {
    recent_posts: Vec<Post>,
}

enum Action {
    AddPost(Post),
}

struct Post {
    uuid: String,
    title: String,
    body: String,
}

fn main() {

    let handle_action = |action: Action| {
        println!("itworks");
    };

    shared_structs::run_action_relay("andy", "asdf", "127.0.0.1:9090".parse().unwrap(), Vec::new(), &handle_action);
}
