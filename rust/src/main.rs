// fn main() {
//     // let x: i32 = 1;
//     // println!("{}", x);
  
//     // let is_male: bool = false;
//     // let is_above_18: bool = true;
    
//     // if is_male {
//     //     println!("You are a male");

//     // } else {
//     //     println!("You are not a male");
//     // }

//     // if is_male && is_above_18 {
//     //     print!("You are a legal male");
//     // }

//     // let greeting: String = String::from("hello world");
//     // println!("{}", greeting);


//     // let is_even: bool = true;

//     // if is_even {
//     //     println!("The number is even");
//     // }
//     // else {
//     //     println!("The number is odd");
//     // }

//     let sentence: String = String::from("The quick brown fox jumps over the lazy dog");
//     let first_word :String =  get_first_word(sentence);

//     print!("first word is: {}" , first_word);
// }


// fn get_first_word(sentence: String) -> String {
//     let mut ans: String = String::from("");

//     for char in sentence.chars() {
//         ans.push_str(char.to_string().as_str());
//         if char == ' ' {
//             break;
//         }
//     }
//     return ans;
// }


// fn stack_fn(){
//     let a: i32 = 10;
//     let b: i32 =20;
//     let c: i32 = a+b;
//     println!("Stack function: the sum of the {} and {} is {}", a, b, c);
// }

// fn heap_fn(){
//     // create a strubg which is allocated on the heap
//     let s1: String = String::from("Hello"); 
//     let s2: String = String::from("World");
//     let combined: String = format!("{} {}", s1, s2);
//     println!("Heap function : Combined string is {}" , combined);
// }

// fn update_string(){
//     let mut s: String = String::from("Initial String");
//     println!("Before update {}", s);
//     println!("Capacity {}", s.capacity());
//     println!("Length {}", s.len());
//     println!("Pointer {:p}", s.as_ptr());

//     s.push_str("and some additional text");

//     println!("After update {}", s);
//     println!("Capacity {}", s.capacity());
//     println!("Length {}", s.len());
//     println!("Pointer {:p}", s.as_ptr());

// }

// fn main() { 
//     let s1 = String::from("Hi there");
//     let s2: String = s1;

//     println!("{}", s2);
// }


fn main() {
    let s1: String = String::from("hello");
    let s2: String = takes_ownership(s1);
    println!("{}", s2);
}


fn takes_ownership(some_string: String) -> String {
    println!("{}", some_string); 
    return some_string; // return the string ownership back to the original main fn
}