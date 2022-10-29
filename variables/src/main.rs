// fn main() {
//     let x = 5;
//
//     let x = x + 1;
//
//     {
//         let x = something(true);
//         println!("The value of x in the inner scope is: {x}");
//     }
//
//     println!("The value of x is: {x}");
//
//     for number in (1..4).rev() {
//         println!("{number}!");
//     }
// }
//
// fn something(condition: bool) -> u32 {
//     if condition {
//         5
//     } else {
//         6
//     }
// }

fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1);
}
