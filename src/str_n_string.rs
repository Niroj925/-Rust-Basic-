fn main(){
    let s:&str="Love you Nepal";
    println!("{}",s);

    //& can be used to conver Box<> type to & str
    //box is heap allocated
    // let x:Box<str>="Hello Nepal".into();//into will conver type annoting which has been annoted in variable
   let x:&str="Hello Nepal";
    greeting(x);//pass reference 

  /////let's dive into string
//string type is defined in std and store as vector of bytes () Box
//but valid always to be format utf-8 sequende
//string is allocated in heap memeory location
let mut st:String=String::from("Hello Nepal ");
st.push_str("what'up");//mutate string 
println!("{}",st);

//replace is used to replace substring
let mut s:String=String::from("i love dog");
//this will allocate new memeory and stored modified
let ss1=s.replace("dog","tiger");
println!("{}",s);


let  s1:String=String:: from("hello");
let s2:String=String:: from("world");
// let s3=s1+s2.as_str();//conver string -> &str we can do like this 
let s3=s1+&s2;
println!("{}",s3);

//& str can converted into string in two ways
let sst:&str="Hello nepal";
println!("{}",sst.to_string());//& str to string
println!("{}",String::from(sst));//& str to string

let str1:String="Mero Nepal".to_string();
let str2:&str=&str1;//&string -> &str
let str3:&str=str1.as_str();
println!("{}",str2);

//get index of string but can not be indexed so we can do slice from ref
let ch:String=String::from("@wow Nepal");
// let sl1=&ch[0];//string can not be indexed
let sl:&str=&ch[0..1];
println!("{}",sl);
let l2:&str=&ch[3..6];
println!("{}",l2);
//string directly can not indexed so convert it into the char
for c in ch.chars() {
    println!("{}",c);
}

}


fn greeting(s:&str){
    println!("{}",s);
}