use std::collections::LinkedList;

pub struct Stack {
    pub list: LinkedList<char>,
}

impl Stack {
    pub fn display(&self) {
        for i in self.list.iter() {
            print!("{i}");
        }
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    pub fn pop(&mut self) -> char {
        if self.list.pop_front() != None {
            self.list.pop_front().unwrap()
        } else {
            '\0'
        }
    }

    // pub fn top(&self) -> char {
    //     *self.list.front().unwrap()
    // }
}
