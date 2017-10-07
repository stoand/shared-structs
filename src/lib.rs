#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

struct User {
    name: String,
    posts: Vec<Post>,
}

struct Post {
    title: String,
    body: String,
}

fn posts() {

}