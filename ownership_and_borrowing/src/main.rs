fn main() {
    // println!("Hello, world!");
    /* - Stack: first in, last out
        + Pushing onto the stack
        + Popping off the stack
       - Heap
        + Allocating on the heap (when you put data on the heap) or sometimes ad as just allocating (pushing values onto the stack is not considered allocating)

    */

    // let mut str = String::from("Hello");
    // str.push_str(", world");
    // println!("The value of str is {str}")

    // let mut s1 = String::from("Hello");
    //
    // let length = give_ownership(&mut s1);
    // // println!("The value of s1 is {s1}");
    // println!("The length of {} is {}", &s1, length);

    // let _r1 = &mut s1;
    // let _r2 = &mut s1;
    // println!("{}", _r1);
    // println!("{}", _r2);

    // let mut s = String::from("Hello World");
    // let word = first_word(&s);
    // println!("log_1 {word}");
    // // s.clear();
    // let size_of_s = s.len();
    // println!("log_2 {size_of_s}");
    // println!("log_3 {word}");
    // s.clear();
    // let size_of_s = s.len();
    // println!("log_4 {size_of_s}");

    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);

    let num = [1,2,3,4,5];
    let slice = &num[1..3];
    let a = slice;
    println!("Slice array {}", a[0]);
    assert_eq!(slice, &[2, 3])
}


// fn give_ownership(_s: &mut String) -> usize {
//     _s.push_str(", world");
//     return _s.len();
// }
//
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return  &s[0..i];
        }
    }
    return &s[..];
}