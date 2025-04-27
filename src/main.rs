// Fonksiyonlar
use std::io;

fn my_first_func(){
    // Fonksiyonları istediğimiz yerde declare edebiliriz tabiki scope dikkat etmek şartıyla

    println!("Hello from my first func!");
}

fn main() {
    my_first_func();

    another_func(123, '😘');

    let my_block = {
        let mut x = 2110;
        x + 21  // Sonuna oktallı virgül koymadığımız için x + 21 ifadesi return edilir. My_block ise assigment operatörü ile bu return edilen değeri yakalar. Let ile tanımlandığı için bir expression'dur. 
    };

    println!("my block equals to {}", my_block);


    let my_2 = {
        let mut x:u64 = 21;

        fn take_root(y:u64)-> u32{   // u32'ye çevirdik çünkü .pow() sadece u32 veri tipi kabul eder.
            y.isqrt() as u32  // isqrt() ise u64 döndürür.
        }

        let rooter = take_root(144);


        x.pow(rooter)

    };

    println!("my_2 equals to {}", my_2);

    // Call Functions

    let value = string_return_func();

    println!("{}", value);

    // 

    println!("{}", str_return_func());


    {
        

        fn my_func()-> &'static str{
            let my_string:&str = "Merhabalarsss"; // my_string read-only memory'deki bir adrese refer eder.
            return my_string;
        }

        let my_string_from_outer_scope = my_func();

        println!("{}", my_string_from_outer_scope);


    }


}


fn another_func(x:i32, my_val:char){
    println!("my value :{} and my char = {}", x, my_val);
}


fn string_return_func() -> &'static str{
    let x = 25;
    let y: i32 = 175;
    let z = x + y;
    println!("the sum up inside of the function {x} + {y} = {z}", );
    return "this function must have returned a &'satatic str because when the function's scope end, string literal does not continue themselves life. Therefore, we need to define it as a static value."
}

fn str_return_func() -> String{
    let my_str: String= String::from("merhaba");
    let _ = my_str.as_bytes();
    return my_str;
    //asd
}