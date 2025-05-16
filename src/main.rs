// Fonksiyonlar

use std::{io, process::Output, mem};
mod str_data;
mod fonksiyonlar;
use fonksiyonlar::function_basics::{another_func,string_return_func,str_return_func,my_generic_func,my_function,my_function_basics, type_set};
use str_data::str_val::modul_usage;  // Modülümüz içindeki fonksiyonları bu şekilde imprt ediyoruz ve kullanmaya hazır hale getiriyoruz.




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

    // Generic func

    my_generic_func(3.12);



    // Life-time for &str variables and fuction scope

    {
        

        fn my_func()-> &'static str{ // &'static str ifadesi bu veri programın ömrü boyunca yaşayacak demektir.
            let my_string:&str = "Merhabalarsss"; // my_string read-only memory'deki bir adrese refer eder.
            return my_string;
        }

        let my_string_from_outer_scope = my_func();

        println!("{}", my_string_from_outer_scope);


    }


    // Type Definiiton and Translation 

    {
        let value = type_set(4.2110);

        println!("Dönüşmüş verinin değeri şudur; {}",value);
    }


    // Fonksiyon Tipleri ( const, async, safe | unsafe,  extern )

        // * extern "Rust" fn <identifier> () kesinlikle noemal fonksiyon tanımlamakla aynı şeydir.

    {
        // safe 
        my_function();

        // unsafe func.

            // Unsafe fonksiyonlar unsafe blokları içinde çağırılmalıdır.

       unsafe {
        my_function_basics();
       } 
    }


    // Check Memory Size for any value

    {
        // Check integer

        let check_x:i32 = 2110000;
        let mem_size = mem::size_of_val(&check_x);

        println!("check_x'in memory boyutu = {}",mem_size); // 4 byte döner çünkü i32 = 8 * 4 byte;

        // Check String
        let cehck_dynamic_str : String = String::from("Merhaba Dünya");
        let mem_size_str: usize = mem::size_of_val(&cehck_dynamic_str);

        println!("bu string'in bellekteki boyutu ise = {} byte", mem_size_str);


        // Check Function

        fn check_memory(){}
        let foo:fn() = check_memory;  // Burada tip bildirimi yapıyoruz ve artık bu arkadaşa referans etmesi gerektiğini söylüyoruz.

        println!("foo'nun neğeri şuna eşit = {:?}", foo);

        // çıktı: foo'nun neğeri şuna eşit = 0x102427724



        // Check Function - 2

        fn check_func_mem(){}
        let bar = check_func_mem;

        // let mem_size_func = &check_func_mem;

        let mem_size_func = &bar;

        println!("bu fonksiyon içinde hiç bir şey olmadığı için 0 bayt yer kaplar. Dolayısıyla bu fonksiyonun bellekteki boyutu = {:#?}", mem_size_func());
    }


    // Dışarıdaki bir modülden fonksiyon import etme ve kullanma

    {
        let modülden_gelen = str_data::str_val::my_str_rtn();
        println!("bu String bize dışardaki bir modülden import edilen String veriyi içeriyor; {}",modülden_gelen)
    }


    // Dışarıdaki bir modülden use keywordü ile import etme ve kullanma

    {
        let modulden_gelen_veri = modul_usage();

        println!("{}", modulden_gelen_veri);
    }


    

    // &str genel


    let message = "hello-world"; // programın çalıştıralabilir ( binary ) dosyasına program ömrü boyunca gömülür.
    let prt = message.as_ptr();
    let lentgh_message = message.len();

    println!("message değişkenin tipi &str'dir ve pointer ile uzunluk bilgisi taşır bunlar sırasıyla =; {:?}, {}", prt,lentgh_message);



    // str_val.rs Dosyası Alanı

    {
        str_data::str_val::string_meta_data();
    }





}


