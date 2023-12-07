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

    pub fn iter_mut(&mut self) -> IterMut<T> {
        if let Some(first) = &mut self.first {
            return IterMut::new(Some(first));
        }

        IterMut::new(None)
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

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        return self.iter();
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut LinkedListNode<T>>,
}

impl<'a, T> IterMut<'a, T> {
    pub fn new(first_node: Option<&'a mut LinkedListNode<T>>) -> IterMut<T> {
        IterMut { next: first_node }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<&'a mut T> {
        if let Some(node) = self.next.take() {
            if let Some(next) = &mut node.next {
                self.next = Some(next);
            }

            return Some(&mut node.value);
        }

        None
    }
}

impl<'a, T> IntoIterator for &'a mut LinkedList<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        return self.iter_mut();
    }
}

pub struct IntoIter<T> {
    next: Option<LinkedListNode<T>>,
}

impl<T> IntoIter<T> {
    pub fn new(first_node: Option<LinkedListNode<T>>) -> IntoIter<T> {
        IntoIter { next: first_node }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if let Some(node) = self.next.take() {
            if let Some(next) = node.next {
                self.next = Some(*next);
            }

            return Some(node.value);
        }

        None
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self.first.map(|x| *x))
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

    #[test]
    fn count_is_zero_on_creation() {
        let list: LinkedList<i32> = LinkedList::new();
        assert_eq!(list.count(), 0);
    }

    #[test]
    fn iter_gives_none_on_empty() {
        let list: LinkedList<i32> = LinkedList::new();
        assert_eq!(list.iter().next(), None);
    }

    #[test]
    fn iter_mut_gives_none_on_empty() {
        let mut list: LinkedList<i32> = LinkedList::new();
        assert_eq!(list.iter_mut().next(), None);
    }

    #[test]
    fn iter_into_gives_none_on_empty() {
        let list: LinkedList<i32> = LinkedList::new();
        assert_eq!(list.into_iter().next(), None);
    }

    #[test]
    fn iter_mut_add_one_to_all() {
        let mut list = LinkedList::new();
        list.add_first(3);
        list.add_first(2);
        list.add_first(1);

        list.iter_mut().for_each(|x| *x += 1);

        let collected: Vec<_> = list.iter().copied().collect();
        assert_eq!(collected, vec![2, 3, 4]);
    }

    #[test]
    fn iter_into_does_its_magic() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.add_first(3);
        list.add_first(2);
        list.add_first(1);

        for (index, item) in list.into_iter().enumerate() {
            assert_eq!(index as i32 + 1, item);
        }
    }
}
