use std::mem;

struct Item<T> {
    value: T,
    next: Option<Box<Item<T>>>,
}

struct List<T> {
    first_item: Option<Box<Item<T>>>,
    last_item: Option<Box<Item<T>>>,
    count: i64,
}

impl<T> Item<T> {
    fn new_in_box(value: T) -> Box<Self> {
        Box::new(Self { value, next: None })
    }

    fn connect_next(&mut self, next: Option<Box<Self>>) {
        self.next = next;
    }
}

impl<T> List<T> {
    fn add_once(&mut self, item: Option<Box<Item<T>>>) {
        self.first_item = item;
        self.last_item = None;
        self.count = 1;
    }

    fn append_wrap(&mut self, item: T, op: impl FnOnce(&mut Self, Option<Box<Item<T>>>)) {
        let item = Some(Item::new_in_box(item));
        if self.count == 0 {
            self.add_once(item);
            return;
        }
        op(self, item);
        self.count += 1;
    }
}

macro_rules! add_once_guard {
    ($self: ident, $item: ident) => {
        if $self.count == 0 {
            $self.add_once($item);
            return;
        }
    }
}

impl<T> super::List for List<T> {
    type Item = T;

    fn new() -> Self {
        Self { first_item: None, last_item: None, count: 0 }
    }

    fn append_last(&mut self, item: Self::Item) {
        self.append_wrap(item, |_self, item| {
            _self.last_item.as_mut().unwrap().connect_next(item);
            _self.last_item = item;
        })
    }

    fn append_first(&mut self, item: Self::Item) {
        self.append_wrap(item, |_self, mut item| {
            mem::swap(&mut item, &mut _self.first_item);
            _self.first_item.as_mut().unwrap().connect_next(item);
        })
    }

    fn first(&self) -> &Self::Item {
        todo!()
    }

    fn last(&self) -> &Self::Item {
        todo!()
    }
}