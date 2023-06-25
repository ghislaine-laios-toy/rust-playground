mod linked_list;

trait List {
    type Item;

    fn new() -> Self;
    fn append_last(&mut self, item: Self::Item);
    fn append_first(&mut self, item: Self::Item);
    fn first(&self) -> &Self::Item;
    fn last(&self) -> &Self::Item;
}
