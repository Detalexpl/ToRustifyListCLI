trait Id {
    fn get_id(&self) -> usize;
}
struct ToDoValue {
    name: String,
    id: usize,
    sub: Option<Vec<Sub>>,
    done: bool,
}

impl ToDoValue {
    fn create_id<G: Id, T: AsRef<Vec<G>>>(list: T) -> usize {
        let used: std::collections::HashSet<usize> = list.as_ref().iter().map(Id::get_id).collect();

        (0..).find(|id| !used.contains(id)).unwrap()
    }
}
impl Id for ToDoValue {
    fn get_id(&self) -> usize {
        self.id
    }
}
struct Sub {
    name: String,
    done: bool,
    id: usize,
}
