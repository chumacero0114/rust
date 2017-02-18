#![plugin(clippy)]
#![feature(plugin)]

#![allow(dead_code, items_after_statements, never_loop)]
#![deny(unused_label)]

fn unused_label() {
    'label: for i in 1..2 {
        if i > 4 { continue }
    }
}

fn foo() {
    'same_label_in_two_fns: loop {
        break 'same_label_in_two_fns;
    }
}


fn bla() {
    'a: loop { break }
    fn blub() {}
}

fn main() {
    'a: for _ in 0..10 {
        while let Some(42) = None {
            continue 'a;
        }
    }

    'same_label_in_two_fns: loop {
        let _ = 1;
    }
}
