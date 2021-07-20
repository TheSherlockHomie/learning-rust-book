fn main() {
    /*
    scalars
    */

    //integers
    let x = 1;
    let y = 2;

    println!("The sum of {} and {} is {}", x, y, x+y);

    //floating point numbers
    let x: f32 = 1.1;
    let y: f32 = 2.1;

    println!("The sum of {} and {} is {}", x, y, x+y);

    //booleans
    let x = true;
    println!("the value of x is {}", x);

    //characters (store UTF-8)
    let c = '😁';
    println!("c is an emoji! {}", c);

    /*
    compound types
    */

    //tuples
    let tup = (500, 6.4, 1);
    let (i, f, g) = tup;

    let i = tup.1;
    let f = tup.2;
    let g = tup.0;

    println!("i is {}, f is {}, g is {}", i, f, g);

    //arrays
    let a = ["do", "re", "mi", "fa", "so", "la", "ti"];
    let months: [&str; 12] = ["January", "February", "March",
                                "April", "May", "June", "July",
                                "August", "September",
                                "October", "November",
                                "December"];

    let repeated = [4.2; 7];

    println!("music {}; month {}; float {}", a[3], months[7], repeated[6]);
}