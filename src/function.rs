fn main(){
    let (x,y)=(3,8);//tuple

    println!("sum:{}",sum(x,y));

    println!("{:?}",print());

    // get_option(2);

    //pattern matching
    let b:bool=false;
    let _v=match b{
        // true=>1,
       true=>{
        println!("This is true");
       },
       false=>{
        println!("This is false");
        panic!("not found false ,but have panic");
       }
    };


}

fn sum(x:i32,y:i32)->i32{
x+y
}

fn print(){ //default return type ()
    println!("This will return unit type");
}


//diverging function
fn get_option(tp:u8)->Option<i32>{

    match tp {
         (tp1) => {
            println!("this match 1")
         }
         2=>{
            println!("this match 2")
         }
    }
    never_return()
}

fn never_return()->!{
    // panic!()
    // unimplimented!()
    todo!()
}
