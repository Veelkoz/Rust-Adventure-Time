
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let mut space = "             ";
    //space = space.len();
    println!("{space}");


    let tup: (char, u32, f64) = ('a', 8, 55.0);

    let f = tup.1;
    println!("{f}");
    another_function();

}


fn another_function(){
    println!("Hiiii");
}