use std::{
    borrow::BorrowMut,
    collections::HashMap,
    fmt::Display,
    iter::Enumerate,
    ops::Index,
    process::ExitStatus,
    str::{Chars, FromStr},
};

pub fn roman_numerals(n: i32) -> String {
    let numeral_mapping = HashMap::from([
        ("I", 1),
        ("IV", 4),
        ("V", 5),
        ("IX", 9),
        ("X", 10),
        ("XL", 40),
        ("L", 50),
        ("XC", 90),
        ("C", 100),
        ("CD", 400),
        ("D", 500),
        ("CM", 900),
        ("M", 1000),
    ]);

    let reverse_numeral: HashMap<i32, &str> = numeral_mapping
        .clone()
        .into_iter()
        .map(|(k, v)| (v, k))
        .collect();

    let mut divisors: Vec<i32> = numeral_mapping
        .into_iter()
        .map(|(_k, v)| v)
        .collect::<Vec<i32>>();
    divisors.sort();
    divisors.reverse();

    let mut division: i32;
    let mut remainder: i32 = n;

    let mut final_numeral: String = String::new();

    for divisor in divisors {
        if remainder == 0 {
            break;
        };
        division = remainder / divisor;

        if division > 0 {
            let _ = &final_numeral.push_str(
                String::from(reverse_numeral[&divisor])
                    .repeat(usize::try_from(division).unwrap())
                    .as_ref(),
            );
        }

        remainder = remainder % divisor;
    }

    return final_numeral;
}

///////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

pub fn longest_palindrome(s: String) -> String {
    let mut final_vec = Vec::<char>::new();
    let mut current_vec = Vec::<char>::new();

    for (i, _char) in s.chars().enumerate() {
        for char in s[i..].chars() {
            (&mut current_vec).push(char);
            if (&current_vec)
                .into_iter()
                .eq((&current_vec).into_iter().rev())
            {
                if &current_vec.len() > &final_vec.len() {
                    final_vec = current_vec.clone()
                }
            };
        }

        current_vec = Vec::<char>::new();
    }

    final_vec.into_iter().collect()
}

///////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////
//Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn add_node(&mut self, val: i32) -> &mut Box<ListNode> {
        let new_node = Box::new(ListNode::new(val));
        self.next = Some(new_node);
        match &mut self.next {
            Some(value) => value,
            None => panic!(),
        }
    }
}

pub fn show_nth_element_to_remove_from_end(
    head: Option<Box<ListNode>>,
    n: i32,
) -> Option<Box<ListNode>> {
    // This is a dogshit read-only implementation that does not actually remove the node,
    // and is impossible to addapt as a 1-2-1 change to mutable breaks the  code.
    let array_size = (n + 1) as usize;
    let mut store_ptrs = Vec::new();
    let mut current_list_elem = &head;
    let mut circular_array_counter: usize = 0;

    loop {
        match current_list_elem {
            Some(node) => {
                if circular_array_counter < array_size {
                    store_ptrs.push(current_list_elem);
                } else {
                    store_ptrs[circular_array_counter % array_size] = current_list_elem;
                }
                circular_array_counter += 1;
                current_list_elem = &node.next;
            }
            None => break,
        }
    }

    println!(
        "the element prior to the one I want to remove {:?}",
        &(store_ptrs[(circular_array_counter - array_size) % array_size])
            .as_ref()
            .unwrap()
            .val
    );

    return head;
}

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut head = head.clone();
    let mut fast = head.clone();
    let mut slow = &mut head;
    let mut index = 0;
    loop {
        match fast {
            Some(node) => {
                fast = node.next;
            }
            None => {
                let aux = &mut slow;
                match aux.as_mut() {
                    Some(node) => {
                        //This does the actual skip as it marks reaching
                        //the end of the list
                        node.next = node
                            .next
                            .get_or_insert(Box::new(ListNode::new(-1)))
                            .next
                            .clone();
                    }
                    None => todo!(),
                }
                break;
            }
        }
        if index > n {
            // This only starts moving once the fast pointer is already doing the
            // lookahead with the appropriate shift
            println!("{:?}", index);
            slow = &mut slow.get_or_insert(Box::new(ListNode::new(-1))).next;
        }
        index += 1;
    }
    println!("{:?}", head.clone());
    head
}

///////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

pub fn reverse(x: i32) -> i32 {
    // This is catastrphic in the event of a very big number close to the bounds of
    // i32, need to rethink this solution
    let mut value = x.clone();
    let mut divisor = 1;
    let mut multiplier = 1;
    let mut final_value = 0;

    let s = if x.is_negative() { -1 } else { 1 };

    value = value;

    if s == 1 {
        while divisor * 10 <= value {
            divisor = divisor * 10;
            if divisor.overflowing_mul(10).1 == true {
                return 0;
            }
        }
        while value > 0 {
            final_value += (value / divisor) * multiplier;
            value = value % divisor;
            divisor = divisor / 10;
            multiplier = multiplier * 10;
        }
    } else {
        divisor = -1;
        while divisor * 10 >= value {
            divisor = divisor * 10;
            if divisor.overflowing_mul(10).1 == true {
                return 0;
            }
        }
        while value < 0 {
            final_value += (value / divisor) * multiplier;
            value = value % divisor;
            divisor = divisor / 10;
            multiplier = multiplier * 10;
        }
    }

    final_value = final_value * s;

    println!("{:?}", final_value);

    return final_value;
}

///////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut char_list: Vec<Chars> = (&strs).into_iter().map(|x| x.chars()).collect();

    let mut prefix: String = String::from_str("").unwrap();

    let mut exit = false;

    while !exit {
        let mut current_char: Option<char> = None;
        for entry in &mut char_list {
            match entry.next() {
                Some(expr) => {
                    if current_char.is_some() {
                        if expr != current_char.unwrap() {
                            exit = true;
                            break;
                        }
                    } else {
                        current_char = Some(expr);
                    }
                }
                None => {
                    exit = true;
                    break;
                }
            }
        }
        if exit != true {
            prefix.push(current_char.unwrap());
        }
    }

    prefix
}
///////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

fn main() {
    // longest_palindrome(String::from("cbbdiiivvvvvvvkkk"));
    // roman_numerals(58);

    // let mut list_node = Box::new(ListNode::new(1));
    // let _ = &list_node
    //     .add_node(2)
    //     .add_node(3)
    //     .add_node(4)
    //     .add_node(5)
    //     .add_node(6)
    //     .add_node(7)
    //     .add_node(8);
    //
    // remove_nth_from_end(Some(list_node), 6);
    // reverse(-2147483412);
    longest_common_prefix(vec![
        String::from_str("booooo").unwrap(),
        String::from_str("boor").unwrap(),
    ]);
}
