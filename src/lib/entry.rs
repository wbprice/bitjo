pub trait Entry {
    fn text(&self) -> String;
    fn insert(&mut self, entry: Box<dyn Entry>);
}
