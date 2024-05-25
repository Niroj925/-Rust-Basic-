fn main(){
    //touple is a collection of various types of variable
    //common signature for touple is T,T1,T2...
    let t1:(i32,u16,&str,String)=(-15,5,"nepal",String::from("ramro xa"));
    println!("{:?}",t1);

    let t2:(u16,(i32,&str))=(4,(15,"Niroj"));
    println!("{:?}",t2);

    //member can be extracted from tuple using indexing
    let t3:(&str,&str,&str)=("Mero","Nepal","Ramro");
    assert_eq!(t3.1,"Nepal");
    println!("success");

    //let's try long tuple
    let t4=(2,3,5,4,8,7,5,6,3,7,3,4);//up to 12 it can hold
    println!("{:?}",t4);

    //destructuring with tuple pattern
    let t4:(&str,i32,f32)=("niroj",-4,6.9);
    let (x,y,z)=t4;
    println!("{},{},{}",x,y,z);

    //tuples as function argument
    let (x,y)=sum_mul((2,7));
    println!("{},{}",x,y);
}

fn sum_mul(num:(i32,i32))->(i32,i32){
    (num.0+num.1,num.0*num.1)
}