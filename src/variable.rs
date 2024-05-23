#[allow(unused_variables)] //for unsued variable allow
fn main() {
    let _x: i32 = 5; //for unused variable use _
    let mut y: i32 = 5; //i32 mean unsign intiger
    y += 4;
    assert_eq!(y, 9);
    println!("success");
    //in rust by default variable are immutable to make variable mutale use keyword mut

    //let's dive into scope
    let num: i32 = 15;

    {
        let num: i32 = 10; //this is shadowing
        println!("The value inside this scope is {}", num);
    }
    println!("The value outside the scope is {}", num);

    //call a function
    greet("Niroj");

    let sqrval = sqr(5);
    println!("sqare value {}", sqrval);

    let mut num = 10;
    incr(&mut num, 5); //pass reference of num variable
    println!("address of num {:?}", &num as *const i32);
    println!("Incr a number {}", num);

    //shadowing
    let mut nn: i32 = 5;

    nn = 6;

    let nn: &str = "Hello Nepal";

    println!("{}", nn);

    //let's practice tuple

    let (mut num1, num2) = (5, 6);
    num1 += 5;

    assert_eq!(num1, 10);
    assert_eq!(num2, 6);
    println!("Nice tuple");

    //destructure tuple
    let (x, y);

    [x, ..] = [3, 6];
    [.., y] = [6, 9];

    assert_eq!([x, y], [3, 9]);
    println!("destructure variabls are equal");
}

fn greet(name: &str) {
    // let x:&str="Nepal";

    println!("Namaster {}", name);
}

fn sqr(num: i32) -> i32 {
    let sqr = num * num;
    return sqr;
}

fn incr(x: &mut i32, incrby: i32) {
    // let address_of_x = x as *const i32;
    println!("address of x {:?}", x as *const i32);
    *x += incrby; //dereferencing using pointer
}
