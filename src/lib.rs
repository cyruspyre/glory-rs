use std::{
    fmt::{Display, Formatter, Result},
    ptr::null_mut,
};

pub struct Glory {
    len: usize,
    idx: Index,
    cur: *mut Node,
    head: *mut Node,
    last: *mut Node,
    is_last: bool,
}

impl Glory {
    pub fn new() -> Self {
        Self::build(String::new())
    }

    pub fn push(&mut self, str: &str) {
        self.len += str.len();
        unsafe { (*self.last).data += str };
    }

    pub fn push_front(&mut self, str: &str) {
        let node = unsafe { &mut *self.head };
        let is_cur = self.head == self.cur;

        if node.data.len() <= 300 {
            node.data.insert_str(0, str);
        } else {
            self.head = Node::new(str.into(), null_mut(), node);
            node.prev = self.head;

            if is_cur {
                self.cur = self.head
            }
        }

        self.idx = Index::new();
        self.len += str.len();
    }

    pub fn insert(&mut self, pos: usize, str: &str) {
        if str.len() == 0 {
            return;
        }

        // println!("{pos}");

        // if pos == 0 {
        //     return self.push_front(str);
        // }

        // if pos == self.len {
        //     return self.push(str);
        // }

        let node = self.locate(pos);

        // if node.data.len() <= 200 && false {
        //     return node.data.insert_str(self.idx.local, str);
        // }

        let data = node.data.split_off(self.idx.local);

        node.data += str;

        let lol = data.len();

        if data.len() != 0 {
            // self.idx.local -= 1;
            // println!("{} {} {}", node.data.len(), data.len(), self.idx.local);
            node.next = Node::new(data, node, node.next);
            // Try resetting it regardless of value if UB
            if self.is_last {
                self.is_last = false;
                self.last = node.next;
            }
        }

        // if node.data.is_empty() || lol == 0 {
        //     println!("{self}\n{} {}", node.data.len(), lol);
        //     panic!()
        // }

        self.len += str.len();
    }

    fn locate<'a>(&mut self, pos: usize) -> &'a mut Node {
        let [head, last, cur] = unsafe { [&mut *self.head, &mut *self.last, &mut *self.cur] };
        let idx = &mut self.idx;
        let mut rev = false;
        let [a, b] = [
            if idx.cur > pos {
                rev = true;
                idx.cur - pos
            } else {
                pos - idx.cur
            },
            self.len - pos,
        ];
        let mut node = if pos < b {
            if pos < a {
                print!("head");
                idx.local = 0;
                idx.cur = 0;
                rev = false;
                head
            } else {
                print!("cur");
                cur
            }
        } else {
            if b < a {
                // println!("{:?} | {:?} | {:?} | {pos}", head, cur, last);
                idx.local = last.data.len() - 1;
                idx.cur = self.len - 1;
                rev = true;
                print!("last");
                last
            } else {
                print!("cur");
                cur
            }
        };
        self.is_last = self.cur == self.last;

        println!(" {rev} {:?} {:?} [{pos}, {a}, {b}]", node, idx);

        while !node.has(idx, pos, rev) {
            let tmp = if rev { node.prev } else { node.next };

            if tmp.is_null() {
                panic!("logic error");
            }

            node = unsafe { &mut *tmp };
            idx.local = if rev { node.data.len() - 1 } else { 0 }
        }

        self.cur = node;

        println!("{:?} {:?}", idx, node);

        node
    }

    fn build(data: String) -> Self {
        let len = data.len();
        let tmp = Node::new(data, null_mut(), null_mut());

        Self {
            len,
            idx: Index::new(),
            cur: tmp,
            head: tmp,
            last: tmp,
            is_last: false,
        }
    }
}

impl From<&str> for Glory {
    fn from(value: &str) -> Self {
        Self::build(value.into())
    }
}

impl Display for Glory {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut buf = String::with_capacity(self.len);
        let mut tmp = unsafe { &*self.head };

        println!(
            "head {:?} | cur {:?} | last {:?} | {:?}",
            self.head, self.cur, self.last, self.idx
        );

        loop {
            println!("-> {:?} | {:?}", tmp as *const Node, tmp.data);
            buf += &tmp.data;

            if tmp.next.is_null() {
                break;
            }

            tmp = unsafe { &*tmp.next }
        }

        write!(f, "{buf}")
    }
}

#[derive(Debug)]
struct Index {
    cur: usize,
    local: usize,
}

impl Index {
    fn new() -> Self {
        Self { cur: 0, local: 0 }
    }
}

#[derive(Debug)]
struct Node {
    data: String,
    prev: *mut Node,
    next: *mut Node,
}

impl Node {
    fn new<'a>(data: String, prev: *mut Node, next: *mut Node) -> *mut Self {
        Box::into_raw(Box::new(Self { data, prev, next }))
    }

    fn has(&self, idx: &mut Index, pos: usize, rev: bool) -> bool {
        if pos == idx.cur {
            return true;
        }

        let rng = if rev {
            0..=idx.local
        } else {
            idx.local..=self.data.len() - 1
        };
        // println!("{:?}", rng);
        let iter = self.data[rng].chars();

        if rev {
            for c in iter.rev() {
                if idx.cur == pos {
                    break;
                }

                // if idx.local == 0 {
                //     break;
                // }

                idx.cur -= 1;
                idx.local -= c.len_utf8();
            }
        } else {
            for c in iter {
                if idx.cur == pos {
                    break;
                }

                idx.cur += 1;
                idx.local += c.len_utf8();
            }
        }

        idx.cur == pos
    }
}
