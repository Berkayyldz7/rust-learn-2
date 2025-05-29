pub fn reference_understanding(val_1:i32)->i32{
    let mut x_1 = 21_i32;
    let y_1 = &x_1; // y1 burada x1'i borç alıyor. ( y_1 kullanılana kadar x_1 değiştirilemez.)

    let z_1 = *y_1; // Dereferans işlemi y_1'in kullanılmasını sağlıyor ve x_1 artık boşa düşüyor.

    x_1 = z_1 * 10;

    return val_1 * x_1
}

pub fn refence_str()->String{
    let x_str = String::from("Bu veri hepateki veri");
    let y_str = &x_str[5..12];

    return  x_str;
}