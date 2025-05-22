pub fn my_str_rtn() -> String {
    return String::from("Merhaba ilk modülden selam");
}

pub fn modul_usage() -> String {
    return String::from("Bu modül içindeki String'i main.rs içine import etmek istiyorsak alternatif yol olarak use:: keywordü kullanabiliriz.");
}


// String Genel Bilgiler.

/*
1. String'ler Heap Memory bölgesinde saklanır.

2. Her bir String ifade 3 meta bilgiyi içinde tutar. (!! Bu metadata stack'te tutulur. )
  - pointer   ==> Heap Üzerindeki Verinin Başlangıç Adresini Gösterir.
  - length    ==> String'in o andaki uzunluğunu gösterir.
  - capacity  ==> String için ayrılmış toplam bellek kapasitesini gösterir. ( /n karakterleri vs dahil )

3. String ifadenin veri kısmı Heap'te saklanırken değişken kısmı Stack'te saklanır.
  -/ let my_str: String = String::from("hello-world");
  - my_str        --> stack'te saklanır. ( pointer ile )
  - "hello-world" --> heap'te saklanır.

4. String type bir verinin Sahibi olduğunu belirtir. Yani Heap üzerinde tutulan veriyi "okuyabilir, yazabilir, silebilir."
 - Bu çok kritik bir nokta çünkü "&str type" değişkenler "sadece bu veriyi okuyabilir."
 - Yani system call gibi mekanizmaları çağırma becerileri yoktur &str type'lı değişkenlerin,  onlar sadece Heap üzerindeki adresin pointer ve length değerini içeren metadata'lara sahiptir.

 -/ let message = "hello world";
 - let ptr = message.as_ptr();



*/


pub fn string_meta_data(){
    let my_str = String::from("Yaşa Mustafa Kemal Paşa");

    // String Meta Verileri

    let my_str_pointer = my_str.as_ptr();
    let my_str_length = my_str.len();
    let my_str_capacity = my_str.capacity();

    println!("String bir verinin meta dataları şunlardır; {:?},{},{}", my_str_pointer,my_str_length,my_str_capacity)


}


pub fn string_with_capacity(){
    let mut  set_a_capacity_for_string = String::with_capacity(5);
    set_a_capacity_for_string.push_str("abcçdefgh");

    let cpc_str = set_a_capacity_for_string.capacity();
    println!("This New String created by .with_capacity() method and was set 8 but capacity overflowed and rearranged by OS {}, capacity = {}", set_a_capacity_for_string, cpc_str)
}