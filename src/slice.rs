fn main(){
    //slice is same as array but their size is unknown at compile time 
    let ss:[i32;5]=[2,3,6,7,4];
    let slice:&[i32]=&ss[1..3];//type annotation and offset
    assert_eq!(slice,&[3,6]);
    println!("slice success");

    let arr:[char;3]=['a','b','c'];
    let slce:&[char]=&arr[..2];
    println!("{:?}",slce);

    //string slice
    // let strr:String=String::from("Mero Nepal");
    let strr:&str="Mero Nepal";
    let strslice:&str=&strr[2..6];
    println!("{:?}",strslice);

    //conver &string to &str implicitly

    let strng:String=String::from("Hamro Nepal");

    let word=first_word(&strng);
    println!("{:?}",word);

}

fn first_word(s:&str)->&str{
    &s[..1]
}