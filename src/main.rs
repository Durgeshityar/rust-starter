


fn main() {
   
   // u8 -> unassigned intger of 8 bits -> similar to types in TS
    let  num:u8= 5;
   print!("The value stored  stored in num  is{} \n",num); 
   
   // &str -> fixed length string - special memory
   let string_literal:&str =  "Hello, world!";
   print!("The value stored in string_literal is {} \n",string_literal);

   // String -> Dynamic - heap allocated
   let mut string_literal2: String = String::from("Hello how are you");
   string_literal2.push_str("whats up");
   print!("The value stored in string_literal2 is {} \n",string_literal2);

   // Tuple
    let emp_info:(&str, u8) = ("Ramesh",50);
    let emp_name = emp_info.0;
    let emp_age = emp_info.1;

    // destructuring at fly
    let (employe_name, employe_age) = emp_info;
    print!("Employe name = {}, Employe age = {} \n",emp_name ,emp_age);
    
    print_value(5);

    let num1:u8 = 10;
    let num2:u8 = 20;
    let result:u8= add(num1, num2);
    println!("The sum of number is {}", result);


}

fn add(num1:u8, num2:u8) -> u8{
    return num1 + num2;
}

fn print_value(item:u8) {
    println!("item value= {} ", item);
}