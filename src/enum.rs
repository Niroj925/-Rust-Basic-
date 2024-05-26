 enum Number{
        One=0,//starting from 0 
        Two=5,
        Three//this becomes 6
    }

 #[derive(Debug)]
enum Message{
    Quiet,
    Move{x:i32,y:i32},//like struct
    Write(String),//tuple
    Change_color(i32,i32,i32)
}


fn main(){
    println!("{}",Number::One as u8);//return 0
    println!("{}",Number::Three as u8);

    let msg1:Message=Message::Move{x:13,y:45};
    let msg2:Message=Message::Write(String::from("enum"));
    println!("{:?}",msg1);
    println!("{:?}",msg2);

    let msg:Message=Message::Move{x:1,y:1};
    if let Message::Move{x:a,y:b}=msg{
        assert_eq!(a,b);
    }else{
        panic!("do not run this");
    }
    println!{"success"};

    let msgs:[Message;3]=[
        Message::Quiet,
        Message::Move{x:2,y:4},
        Message::Change_color(244,255,0)
    ];
    for msg in msgs{
        show_msg(msg);
    }

//option enum 
//it is just like null in others 
let five:Option<i32>=Some(5);//this is option enum
let six:Option<i32>=plus_one(Some(1));
let none:Option<i32>=plus_one(None);
if let Some(n)=six{
println!("{}",n);
}else{
    panic!("do not run this");
}

}

fn show_msg(msg:Message){
    println!("{:?}",msg);
}

fn plus_one(x:Option<i32>)->Option<i32>{
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}