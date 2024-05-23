use std::mem::size_of_val;
fn main(){
    let c1:char='a';//4 bytes
    println!("{}",size_of_val(&c1));

    assert_eq!(size_of_val(&c1),4);

    let s1="gdgs";//"" that's mean string
    println!("{}",size_of_val(&s1));

    println!("success calc length of character");

    let ch:char='n';
    print_char(ch);

    //boolean
    let t:bool=true;
    if t{
        println!("bool true");
    }

    let b1:bool=false;
    let b2:bool=true && false;//false

    assert_eq!(b1,b2);
    println!("comparasion bool");
    println!("{}",size_of_val(&b1));//1

    //unit type 
    let  _v:()=();//this is unit type return empty

    let v:(i32,i32)=(2,3);

    assert_eq!(_v,implicitly_ret_unit());
    println!("{}",size_of_val(&_v));//0

}

fn print_char(c:char){
    println!("{}",c);
}

fn implicitly_ret_unit(){
println!("This will return unit ()");
}