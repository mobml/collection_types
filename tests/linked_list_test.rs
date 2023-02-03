#[cfg(test)]
use collection_types::LinkedList;

mod test_linked_list {

    use collection_types::list;

    use super::*;

    #[test]
    fn it_works() {
        let lkl = LinkedList::<i32>::new();

        assert_eq!(lkl.len(), 0);
        assert!(lkl.is_empty());
    }

    #[test]

    fn add_elements() {
        let mut lkl = LinkedList::<i32>::new();

        lkl.append(12);

        assert_eq!(lkl.len(), 1);
        assert!(!lkl.is_empty());
    }

    #[test]

    fn add_much_elements() {
        let mut lkl: LinkedList<i32> = LinkedList::new();

        for i in 0..4 {
            lkl.append(i);
        }

        assert_eq!(lkl.len(), 4);
        lkl.push_front(1);
        assert_eq!(lkl.len(), 5);
    }

    #[test]

    fn see_tail() {
        let mut lkl: LinkedList<i32> = LinkedList::new();

        for i in 0..4 {
            lkl.append(i);
        }

        assert_eq!(lkl.len(), 4);
        lkl.push_front(1);
        assert_eq!(lkl.len(), 5);

        assert_eq!(lkl.seek_tail(), Some(3));
    }

    #[test]

    fn see_front() {
        let mut lkl: LinkedList<i32> = LinkedList::new();

        for i in 0..4 {
            lkl.append(i);
        }

        lkl.push_front(1);
        assert_eq!(lkl.seek_head(), Some(1));
    }

    #[test]

    fn it_works_remove_head() {
        let mut lkl = LinkedList::<i32>::new();

        for i in 0..4 {
            lkl.append(i);
        }

        assert_eq!(Some(0), lkl.remove_head());
        assert_eq!(3, lkl.len());
        assert_eq!(Some(1), lkl.seek_head());
    }

    #[test]

    fn it_works_iter() {
        let mut lkl = LinkedList::<i32>::new();

        for i in 0..4 {
            lkl.append(i);
        }

        for i in lkl.iter() {
            println!("{}", i);
        }

        let mut iter = lkl.iter();

        assert_eq!(iter.next(), Some(0));
    }

    #[test]

    fn it_works_list_macro() {
        let list: LinkedList<i32> = list![1, 2, 4];
        let mut list = list.iter();
        assert_eq!(Some(1), list.next());
        assert_eq!(Some(2), list.next());
        assert_eq!(Some(4), list.next());
    }
}
