
use std::collections::HashMap;



fn main() {

    let values = vec![8,6,4,10,2,4,1,0];
    // println!("La mediane est {}", mediane(&values));

    
    println!("The most seen value is {}",mode(&values));

//    let my_str = "je suis dans un monde ou l'on veut feminiser l'homme";
//     println!("{:?}",test(my_str));
}



fn mediane (value: &Vec<i32>) -> f64 {

//    let mut sum:f64 = 0.0;
//     for i in value {
//         sum += *i as f64;
//     }

//     return sum / value.len() as f64

    let sum = value.iter().fold(0, |acc,x| acc + x);
    sum as f64 / value.len() as f64


}


fn mode (value: &Vec<i32>)  -> i32 {
    
    let mut map = HashMap::new();

    for val in value  {
       let count = map.entry(val).or_insert(0);
       *count += 1;
    //    println!("{}", count);  //how many times this value has been seen?
       
    }
    println!("{:?}", map); //the actual HashMap state

    let mut max = 0;   //stores the current greater value
    let mut mode = 0;  // stores the current key attached to the the current greater value
    for (k, v) in map { 
       if v > max { //if the current value > max then max take that value 
           max = v;
           mode = *k; //then the mode is the key attached to that value
       }
    }
    mode

} 

fn test (my_str: &str) -> HashMap<char, i32> { 
    
    let mut letters = HashMap::new();

    for ch in my_str.chars() {
        letters.entry(ch).and_modify(|counter| *counter += 1).or_insert(1);
    }
    
    letters
    

}
   

