
fn main(){
    let arr:[i32;5]=[2,3,8,7,9];
    println!("Length of array :{}",arr.len());
    println!("{:?}",arr);

    let arr0=[2,1,5,7,6];
    let arrc:[_;3]=['a','b','c'];
    type_of(&arrc);
    println!("{}",std::mem::size_of_val(&arrc));

    let num:[i32;100]=[1;100];//init 100 of 1 in array
    println!("{}",num[22]);
    type_of(&num);

    let ch:[char;2]=['x','y'];
    println!("{}",ch[0]);

    let names:[String;2]=[String::from("Niroj"),"Thapa".to_string()];
    //get return option <T>,it is safe to use 
    let name0=names.get(0).unwrap();
    println!("{}",name0);
    //indexing is not safe to use 
    let name1=&names[1];
    println!("{}",name1);
}

fn type_of<T>(_:&T){
    println!("{}",std::any::type_name::<T>());
}