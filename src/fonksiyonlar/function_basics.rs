pub fn another_func(x:i32, my_val:char){
    println!("my value :{} and my char = {}", x, my_val);
}


pub fn string_return_func() -> &'static str{
    let x = 25;
    let y: i32 = 175;
    let z = x + y;
    println!("the sum up inside of the function {x} + {y} = {z}", );
    return "this function must have returned a &'satatic str because when the function's scope end, string literal does not continue themselves life. Therefore, we need to define it as a static value."
}

pub fn str_return_func() -> String{
    let my_str: String= String::from("merhaba");
    let _ = my_str.as_bytes();
    return my_str;
    //asd
}



// GENERİC FUNCTİON

pub fn my_generic_func<T>(deger:T)
where
T:PartialOrd, 
T:Copy,
T:std::ops::Mul<f32, Output = T>
{
    let kopya_deger = deger *  0.1;
    if deger > kopya_deger {
        println!("Değer kopyasından büyük.");
    } else {
        println!("Değer kopyasından büyük değil.");
    }
}

// PartialOrd = Generic bir fonksiyon ürettiğimiz için parametreler her veriyi kabul edebilir bizim ürettiklerimiz dahil. Bu verilerin sıralanabilir olup olmadığını compiler asla bilemez. Armutla elmayı sırala demekle aynı şey. Bizim ona bu verileri sıralayabilirsin komutu vermemizi bekler.

// Copy = Yine yukarıdaki sebep ile aynı durumdan kaynaklı olarak bizim bir fonksiyona verdiğimiz verilerin kopyalanabilir olup olmadığını da ona söylememiz gerekir.


pub fn type_set(val:f64)->u16{

    let x = 4.1234_f32;  // numeric verilerde _ koyarakta type tanımlaması yapabiliyoruz.
    let y = 21_u16;

    let t = val as u16;
    return t*y
}


pub extern "Rust" fn my_function(){
    println!("extern Rust fn my_function() tanımlaması kesinlikle fn my_function() diye tanımlamakla aynı şeydir. ")
}

pub unsafe extern  "Rust" fn my_function_basics(){
    println!("unsafe extern <identifier> fn() tanımlaması da kesinlikle fn <identifier> demekle aynı şeydir ancak artık rust'ın sağladığı bellek güvenliği bu bloklarda geçerli değildir.")
}