use std::collections::HashMap;

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

    fn add_node(&mut self, val: i32) -> &mut Self {
        self.next = Some(Box::new(ListNode::new(val)));
        return self;
    }
}

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut store_ptrs = [-1, n + 1];
    let mut current_list_elem = &head;
    let mut circular_array_counter: i32 = 0;

    loop {
        match head {
            Some(node) => {
                store_ptrs[circular_array_counter] = head;
                circular_array_counter = (circular_array_counter + 1) % (n + 1);
                head = node.next;
            }
            None => expr,
        }
    }

    return head;
}

///////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

fn main() {
    // longest_palindrome(String::from("cbbdiiivvvvvvvkkk"));
    roman_numerals(58);

    let mut list_node = Box::new(ListNode::new(1));
    let _ = &list_node.add_node(2).add_node(3).add_node(4).add_node(5);

    remove_nth_from_end(Some(list_node), 1);
}