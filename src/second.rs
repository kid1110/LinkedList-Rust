use std::rc::Rc;

pub struct IntoIter<T>(List<T>);
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}
pub struct List<T> {
    head: Link<T>,
}
type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }
    pub fn prepend(&self, elem: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                elem,
                next: self.head.clone(),
            })),
        }
    }
    pub fn tail(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }
    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
    pub fn iter_mut(&mut self) -> IterMut<T> {
        match self.head {
            None => IterMut { next: None },
            Some(ref mut node) => IterMut {
                next: Rc::get_mut(node),
            },
        }
    }
}
impl<T> Default for List<T> {
    fn default() -> Self {
        List { head: None }
    }
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.head.take().and_then(|node| {
            if let Ok(mut n) = Rc::try_unwrap(node) {
                self.0.head = n.next.take();
                Some(n.elem)
            } else {
                None
            }
        })
    }
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().and_then(|node| match node.next {
            Some(ref mut rc_node) => {
                self.next = Rc::get_mut(rc_node);
                Some(&mut node.elem)
            }
            _ => Some(&mut node.elem),
        })
    }
}
#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        // Make sure empty tail works
        let list = list.tail();
        assert_eq!(list.head(), None);
    }
    #[test]
    fn iter() {
        let list = List::new().prepend(1).prepend(2).prepend(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
    #[test]
    fn into_iter() {
        let mut list = List::new();
        assert_eq!(list.head(), None);
        list = list.prepend(4).prepend(5).prepend(6);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(6));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn iter_mut() {
        let mut list = List::new().prepend(3).prepend(2).prepend(1);
        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), None);
    }
}
