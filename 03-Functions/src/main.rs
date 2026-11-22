fn main() {
    println!("Hello, world!");
    another_function(3, 'm');
    lift();
}


fn another_function(x: i32, unit: char){
    println!("another hellow");
    println!("{} {unit}",x * 2);
}

fn lift(){
    for number in (1..4).rev(){
        println!("{number}");
    }

    println!("lifts");
}