fn main(){
//borrowing is to take ownership temporarly

//borrowing rules
//1.we can have only one mutable reference of the same data at a time 
//2.

//eg of references
let s=String::from("Hello");
let s1=calc_len(&s);
println!("{},{}",s,s1);

////////////////////mutable references
let mut x=String::from("Hamro Nepal");
// x.push_str(" Dami Nepal");
change_ref(&mut x);
println!("{}",x);

//borrowing rule
let mut  s2=String::from("Hi");
// let r1=&mut s2;
// let r2=&mut s2;//rule 1 violets
// {
//     r3=&mut s2;
// }
let rr1= &s2;
let rr2= &s2;
//this gives error it's already borrowed as immutable so 
//it can be borrowed either as mutable or immutable at same time
// let rr3= &mut s2;
println!("{},{}",rr1,rr2); 
let rr3=&mut s2;//this is valid rr1 and rr2 variable is already used 
println!("{}",rr3);


//////////###########Dangling reference
// let ref_to_no=dangling();

let x:i32=6;
let p:&i32=&x;
println!("memory address {:p}",p);
println!("{},{}",x,*p);//x and p allocate same memmory location 


let mut  xx=String::from("Thapa");
let yy:&mut String=&mut xx;
yy.push_str(" kaji");
println!("{}",xx);

let c:char='n';
let r1:&char=&c;
let ref r2=&c;
println!("{},{}",*r1,*r2);
assert_eq!(get_addr(r1),get_addr(r2));
println!("success");


let mut strng:String=String::from("hlo ");
borrow_obj(&strng);
strng.push_str("k xa");
println!("{}",strng);

let mut s:String=String::from("mero");
let  r1:&mut String=&mut s;
r1.push_str(" nepal");
let  r2:&mut String=&mut s;
r2.push_str(" Ramro xa");
println!("{}",r2);

}
/////////////////////functions///////////////
fn calc_len(s:&String)->usize{
    s.len()
}

fn change_ref(strr:&mut String){
strr.push_str(" Ramro Nepal");
}

//it violets that reference must always valid
// fn dangle()-> &String{
//     let s=String::from("Hello nepal");
//     &s
// }

fn no_dangle()-> String{
    let s=String::from("Hello nepal");
    s
}

fn get_addr(c:&char)->String{
    format!("{:p}",c)//format is same as print it return only string
}

fn borrow_obj(s:&String){}