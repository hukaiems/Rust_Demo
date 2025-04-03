fn main() {
    let s = String::from("hello"); //s bắt đầu có hiệu lực trong scope

    takes_ownership(s);   //s's value chuyển vào trong function
                          // s không sử dụng được nữa vì đã vào function

    let x = 5;  //x đc đưa vào scope

    makes_copy(x); // x là i32 (integer) nên được copy và vẫn có thể sử dụng trong scope mà ko bị free
                   // với các data type đơn giản như booleans hay int sẽ được copy vì ít chi phí

    println!("Output từ println! sau hàm makes_copy(): {}", x); 
    // vẫn có thể gọi x dù đã đưa vào hàm makes_copy()
    // ngoặc nhọn để báo cho println chỗ nên để biến x vào
}                                   

fn takes_ownership(some_string: String) { //some_string vào trong scope
    println!("{some_string}");
} // sau dấu ngoặc, some_string ra khỏi scope và function "drop"
    //  được gọi tự động để free memmory


fn makes_copy(some_integer: i32){
    println!("Output từ hàm makes_copy(): {some_integer}");
}