use std::cell::RefCell;

mod blog;

thread_local! {
    static BLOGS: RefCell<Vec<String>> = RefCell::new(Vec::new());
}
// Title
// Data
// Tagi
// komentarze ?

#[ic_cdk::update]
fn add_blog(new_blog: String) {
    BLOGS.with(|blogs| blogs.borrow_mut().push(new_blog));
}

#[ic_cdk::query]
fn get_blogs() -> Vec<String> {
    BLOGS.with(|blogs| blogs.borrow().clone())
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}