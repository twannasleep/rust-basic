fn main() {
    let mut s = String::from("Hello world");   
    let s2 = &mut s;
    
    s2.push_str(" Twann!");

    println!("s2 {}", s2);
    
    let numbers = vec![1,2,3,4,5,6];
    numbers.iter().for_each(|x| println!("{}", x));
    
    let doubled: Vec<i32> = numbers.iter().map(|&x| x*2).collect();
    println!("{:?}", doubled);
}