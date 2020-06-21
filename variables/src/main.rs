//fn main() {
	  // you must specify mut or x's value cannot be changed
//    let mut x = 5;
//    println!("The value of x is: {}", x);
//    x = 6;
//    println!("The value of x is: {}", x);
//}


// the code panics at runime because index > a.length
fn main() {
    let a = [1, 2, 3, 4, 5];
    let index = 10;

    let element = a[index];

    println!("The value of element is: {}", element);
}