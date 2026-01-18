// https://medium.com/better-programming/learning-rust-building-a-linked-list-102bcb08f93b - Credits

pub struct LinkedList<T> {
    pub val: Option<T>,
    pub next: Option<Box<LinkedList<T>>>,
}