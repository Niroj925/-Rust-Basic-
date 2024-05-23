fn main(){
      //example
      let x:u32=5u32;
      let y:u32={
          let sq:u32=x*x;
          let qb:u32=sq*x;
  
          //this expression is assigned to y 
          x+sq+qb  //without ; that means the value is assgined to y that's like return
      };
  
      let z:()={
          //semicolon supress this expression and assign '()' and return '()' unit type
          x+x;
      };
      println!("x is {:?}",x);
      println!("y is {:?}",y);
      println!("z is {:?}",z);

      let v:u32={
       let mut x:u32=5;
        x*=x;
        x //return x
      };
      println!("{}",v);

      println!("sum:{}",sum(7,6));
}

fn sum(n1:u32,n2:u32)->u32{
    n1+n2
}