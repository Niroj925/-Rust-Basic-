
#[derive(Debug)]//to print stuct we have to use this trait
struct Person{
    name:String,
    age:u8
}

#[derive(Debug)]
struct User{
  name:String,
  email:String,
  contact:u64,
  is_active:bool,
  sign_in_count:u64
}

#[derive(Debug)]
struct File{
name:String,
data:String
}

struct Point(i32,i32,i32);
struct Color(i32,i32,i32);

fn main(){
    let age:u8=2;
  let p:Person=Person{
    name:String::from("Thapa"),
    age
  };
  println!("{}",p.name);
  println!("{:?}",p);

  let v:Point=Point(2,4,36);
  check_Color(v);

  //operating on struct 
  //we can make mutable whole struct but can to single field
  let ag:u8=18;
  let mut p0:Person=Person{
   name:String::from("Nirajan"),
   age:ag
  };
  p0.name=String::from("kaji");
  println!("{:?}",p0);

  //create other instance with instance struct update
 let u0:User=User{
  name:String::from("Nirjan"),
  email:String::from("nn@gmail.com"),
  contact:dbg!(9800898087),//print dbg stderror and assign the value
  is_active:true,
  sign_in_count:45
 };
//  println!("{:?}",u0);
dbg!(&u0);//print debug info to stderr 
let u2:User=set_email(u0);
println!("{:?}",u2);//print debug into to stdout


//partial move
let f1:File=File{
  name:String::from("Readmi.md"),
  data:"this is the file's data".to_string()
};

let _name:String=f1.name;//here _name variable is the owner of the name string data
// println!("{},{},{:?}",_name,f1.data,f1);//it gives error whole instance can not be taken becoz of partial move
//to print whole instance we can clone for name then ownership does not transfer
println!("{},{}",_name,f1.data);
}

fn check_Color(p:Point){
  let Point(x,_,z)=p;
  println!("{}",x);
  println!("{}",p.1);
}

fn set_email(u:User)->User{
  User{
    email:String::from("nirajan@gmail.com"),
    ..u
  }
}