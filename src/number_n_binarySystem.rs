//use package
use std::ops::{Range,RangeInclusive};

fn main(){
    let num:i32=-40;//this is signned number that's can be <0
    println!("{}",num);

    let unum:u16=40_u8 as u16 ;//convert u8 to u16
    println!("{}",unum);


    let x:u32=45;
    assert_eq!("u32".to_string(),type_of(&x));
    println!("success return type");
 
    assert_eq!(i8::MAX,127); //the max value of the sign 8 bit intiger
    assert_eq!(u8::MAX,255);
    println!("success ");

    // let un=251_u8+7;//this gives overflow error max 255 for u8
    let un=251_u16+7;
    println!("{}",un);

    // let un1=i8::checked_add(253,7).unwrap();//i8 ranges from -128 to 127
    let un1=i16::checked_add(253,7).unwrap();
    println!("{},{}",un,un1);

    //add diff type number system
    let vn=1_024+0xff+0o77+0b1111_1111;//decimal,hex,octal and binary number ie 1024+255+63+255
    assert!(vn==1597);
    println!("{}",vn);


    //floting number
    let x1:f64=1_0245.000_1;//??
    let _y1:f32=0.2564;//f32
    let _z1:f64=0.21_f64;//f64
    assert_eq!(type_of(&x1),"f64".to_string());
    println!("{}",type_of(&x1));

    //add float numver
    // assert!(0.1+0.2==0.3);//this gives error floating point precision to prevent this may 0.300000000001
    assert!(0.1_f32+0.2 as f32==0.3_f32);
    println!("success");



    //let do some iteration
    let mut sum:i32=0;//-5
    for i in -3..2{//-3 to 1 end point ie 2 not included
        sum+=i;
        // println!("{}",sum);
    }
    assert!(sum == -5);

    for ch in 'a'..='z'{//a-z =ie include end point
        println!("{} asci value=>{}.",ch,ch as u8);
        
    }
    println!("iteration test success");


    //let test ranges include or not
    assert_eq!((1..5),Range{start:1,end:5});
    assert_eq!((1..=5),RangeInclusive::new(1,5));
    println!("success range include test");


    //operation
    //addition
    assert!(1u32+2u32==3u32);
    println!("addition passed");

     //subtraction
     assert!(1i32-2i32==-1);//by default type is i32
     println!("subtraction passed");

     //multiplication
     assert!(1*2==2);
     println!("multiplication passed");

     //devision
     assert!(1f32/2 as f32==0.5f32);
     println!("division passed");

     //modulo operation
     assert!(14 %3==2);
     println!("modulo passed");
 
     //boolean logic
assert!(true&&false==false);
assert!(true||false==false);
assert!(!true==false);
println!("and logic");


//bitwise operator
println!("0011 AND 1010 is {:04b}",0b0011 & 0b1010 );//multiply 
println!("0011 OR 1010 is {:04b}",0b0011 | 0b1010 );//addition
println!("0011 XOR 1010 is {:04b}",0b0011 ^ 0b1010 );//xor
println!("1<<5 is {}",1<<5);//left shift
println!("0x80>>2 is 0x{:x}",0x80u32>>2);//right shift
 
}

//get the type of the given variable and return the string value
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

