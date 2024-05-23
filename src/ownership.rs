fn main(){
    let x:String=String::from("hello world");//x holds the pointer 
    // let y=x;//this is not valid ownership is unique for a variable at a time
    let y:String=x.clone();//deep copy
    println!("{},{}",x,y);

    let s1:String=take_ownership(x);
    println!("{}",s1);

    println!("{}",x);//this gives error the ownership is taken by s1 and x's ownership is dropped


    //convert sting to bytes vector
    let s:String=String::from("Hello Nepal");
    let byt=s.into_bytes();
    // println!("{}",byt);
}

fn take_ownership(s:String)->String{
//  println!("{}",s);
s
}