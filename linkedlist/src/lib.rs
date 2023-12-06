pub struct LinkedListNode<T> {
    value: T,
    next: Option<Box<LinkedListNode<T>>>,
}

pub struct LinkedList<T> {
    first: Option<Box<LinkedListNode<T>>>,
    count: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            first: None,
            count: 0,
        }
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn add_first(&mut self, value: T) {
        self.first = Some(Box::new(LinkedListNode {
            value,
            next: self.first.take(),
        }));
        self.count += 1;
    }

    pub fn pop_first(&mut self) -> Option<T> {
        if let Some(mut node) = self.first.take() {
            self.first = node.next.take();
            self.count -= 1;
            return Some(node.value);
        }

        None
    }

    pub fn iter(&self) -> Iter<T> {
        if let Some(first) = &self.first {
            return Iter::new(Some(first));
        }

        Iter::new(None)
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        LinkedList::new()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a LinkedListNode<T>>,
}

impl<'a, T> Iter<'a, T> {
    pub fn new(first_node: Option<&'a LinkedListNode<T>>) -> Iter<T> {
        Iter { next: first_node }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if let Some(node) = self.next.take() {
            if let Some(next) = &node.next {
                self.next = Some(next);
            }

            return Some(&node.value);
        }

        None
    }
}

/*#[macro_export]
macro_rules! linkedlist {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_list = LinkedList::new();
            $(
                temp_list.add_last($x);
            )*
            temp_list
        }
    };
}*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut list = LinkedList::new();

        assert_eq!(list.count(), 0);
        assert_eq!(list.iter().next(), None);

        list.add_first(1);
        assert_eq!(list.count(), 1);

        let mut iter1 = list.iter();
        assert_eq!(iter1.next(), Some(&1));
        assert_eq!(iter1.next(), None);

        let mut iter1 = list.iter();
        assert_eq!(iter1.next(), Some(&1));
        assert_eq!(iter1.next(), None);

        list.add_first(2);
        assert_eq!(list.count(), 2);

        let mut iter2 = list.iter();
        assert_eq!(iter2.next(), Some(&2));
        assert_eq!(iter2.next(), Some(&1));
        assert_eq!(iter2.next(), None);

        let mut iter2 = list.iter();
        assert_eq!(iter2.next(), Some(&2));
        assert_eq!(iter2.next(), Some(&1));
        assert_eq!(iter2.next(), None);

        list.add_first(3);
        assert_eq!(list.count(), 3);

        let mut iter3 = list.iter();
        assert_eq!(iter3.next(), Some(&3));
        assert_eq!(iter3.next(), Some(&2));
        assert_eq!(iter3.next(), Some(&1));
        assert_eq!(iter3.next(), None);

        let mut iter3 = list.iter();
        assert_eq!(iter3.next(), Some(&3));
        assert_eq!(iter3.next(), Some(&2));
        assert_eq!(iter3.next(), Some(&1));
        assert_eq!(iter3.next(), None);

        assert_eq!(list.pop_first(), Some(3));
        assert_eq!(list.count(), 2);

        let mut iter2 = list.iter();
        assert_eq!(iter2.next(), Some(&2));
        assert_eq!(iter2.next(), Some(&1));
        assert_eq!(iter2.next(), None);

        let mut iter2 = list.iter();
        assert_eq!(iter2.next(), Some(&2));
        assert_eq!(iter2.next(), Some(&1));
        assert_eq!(iter2.next(), None);

        assert_eq!(list.pop_first(), Some(2));
        assert_eq!(list.count(), 1);

        let mut iter1 = list.iter();
        assert_eq!(iter1.next(), Some(&1));
        assert_eq!(iter1.next(), None);

        let mut iter1 = list.iter();
        assert_eq!(iter1.next(), Some(&1));
        assert_eq!(iter1.next(), None);

        assert_eq!(list.pop_first(), Some(1));
        assert_eq!(list.count(), 0);
        assert_eq!(list.iter().next(), None);
    }
}
