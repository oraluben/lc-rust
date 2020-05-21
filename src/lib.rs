pub mod sol732 {
    use std::collections::HashMap;

    #[derive(PartialEq)]
    enum I {
        Start,
        End,
    }

    struct MyCalendarThree {
        m: HashMap<i32, Vec<I>>,
        l: Vec<i32>,
    }

    /**
     * `&self` means the method takes an immutable reference.
     * If you need a mutable reference, change it to `&mut self` instead.
     */
    impl MyCalendarThree {
        fn new() -> Self {
            MyCalendarThree { m: Default::default(), l: vec![] }
        }

        fn book(&mut self, start: i32, end: i32) -> i32 {
            if !self.m.contains_key(&start) {
                self.l.push(start);
                self.m.insert(start, vec![]);
            }
            if !self.m.contains_key(&end) {
                self.l.push(end);
                self.m.insert(end, vec![]);
            }
            self.l.sort();

            let mut deleted = false;
            let v = self.m.get_mut(&start).unwrap();
            for i in 0..v.len() {
                if v[i] == I::End {
                    deleted = true;
                    v.swap_remove(i);
                    break;
                }
            }
            if !deleted {
                v.push(I::Start)
            }
            deleted = false;
            let v = self.m.get_mut(&end).unwrap();
            for i in 0..v.len() {
                if v[i] == I::Start {
                    deleted = true;
                    v.swap_remove(i);
                    break;
                }
            }
            if !deleted {
                v.push(I::End)
            }


            let (mut r, mut c) = (0, 0);
            for i in &self.l {
                self.m.get(i).unwrap().iter().for_each(|i| {
                    match i {
                        I::Start => c += 1,
                        I::End => c -= 1,
                    }
                });
                if c > r { r = c }
            }
            r
        }
    }

    /**
     * Your MyCalendarThree object will be instantiated and called as such:
     * let obj = MyCalendarThree::new();
     * let ret_1: i32 = obj.book(start, end);
     */

    pub fn test() {
        let mut cal = MyCalendarThree::new();
        assert_eq!(cal.book(10, 20), 1);
        assert_eq!(cal.book(50, 60), 1);
        assert_eq!(cal.book(10, 40), 2);
        assert_eq!(cal.book(5, 15), 3);
        assert_eq!(cal.book(5, 10), 3);
        assert_eq!(cal.book(25, 55), 3);
    }
}
