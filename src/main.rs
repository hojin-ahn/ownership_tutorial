fn main() {
    // 4-3.
    // let s = String::from("hello");

    // takes_ownership(s);

    // let x = 5;

    // makes_copy(x);

    // 4-4.
    // let s1 = gives_ownership();

    // let s2 = String::from("hello");

    // let s3 = takes_and_gives_back(s2);

    // 4-5.
    // let s1 = String::from("hello");

    // let (s2, len) = calculate_length(s1);

    // println!("The length of '{}' is {}.", s2, len);

    // 4-6.
    // let mut s = String::from("hello");

    // change(&mut s);

    // println!("{}", s);

    // 4-7.
    let s = String::from("hello");
    let len = s.len();
    let slice = &s[3..len];
    let slice = &s[3..];
}

// 4-3.
// fn takes_ownership(some_string: String) {
//     println!("{}", some_string);
// }

// fn makes_copy(some_integer: i32) {
//     println!("{}", some_integer);
// }

// 4-4.
// fn gives_ownership() -> String {
//     let some_string = String::from("yours");

//     some_string
// }

// fn takes_and_gives_back(a_string: String) -> String {
//     a_string
// }

//4-5.
// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len();

//     (s, length)
// }

// 4-6.
// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// 4-4.
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
