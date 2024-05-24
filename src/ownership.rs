fn main(){
    let x:String=String::from("hello world");//x holds the pointer 
    // let y=x;//this is not valid ownership is unique for a variable at a time
    let y:String=x.clone();//deep copy
    // print_str(x.clone());
    println!("{},{}",x,y);

    let s1:String=take_ownership(x);
    println!("{}",s1);

    // println!("{}",x);//this gives error the ownership is taken by s1 and x's ownership is dropped

   let new_owner:String=give_owner();
   println!("{}",new_owner);
 
   //use copy instead of clone
   //i32 are has fixed sized and string are dynamic type which in 
   //heap memory allocated  size is not known at compile time 
   let x:(i32,i32,(),&str)=(3,6,(),"Thapa");//use string literal which is fixed size
   let y:(i32,i32,(),&str)=x.clone();
   println!("{:?},{:?}",x,y);

 //this gives error y is used previously and freed memory and 
 //instead of this another type of y variable is assigned 
//    println!("{}",y);

let y:String="hello".to_string();
println!("{}",y);
// println!("{:?}",y);

//mutability can be changed when ownership transfer
let s2:String=String::from("Hamro Nepal");
let mut s3:String=s2; //ownership of the s2 data is transfered to s3 and s2 dropped
s3.push_str(" Ramro Nepal");
println!("{}",s3);

let mut b1: Box<i32>=Box::new(4);//b holds the refrence of the memory location
let b2:Box<i32>=Box::new(5);//b2 return the pointer 
println!("{}",b2);
*b1=6;//change the value in refreence ie mememory location
println!("{}",b1);

//partial move
struct Person{
name:String,
age:Box<u8>
}
let p1:Person=Person{
  name:String::from("thapa"),
age:Box::new(10)
};

let Person{name,ref age}=p1;
println!("{}",name);
// println!("{}",p2)
println!("{}",p1.age);


let t:(String,String)=(String::from("Niroj"),String::from("Thapa"));
let _t1=t.0;//modify particular line only

println!("{:?}",t.1);

let v:(String,String)=(String::from("Thapa"),String::from("Kaji"));
let (var1,var2)=v.clone();
println!("{:?},{:?},{:?}",var1,var2,v);

}


fn print_str(s:String){
println!("{}",s);
}

fn take_ownership(s:String)->String{ 
//  println!("{}",s);
s
}

fn give_owner()->String{
  let s:String=String::from("Hello Nepal");
  let byt=s.as_bytes();//convert string to vector 
  println!("{:?}",byt);//to print list of argument use :? 
  s
}

