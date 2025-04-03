fn main() {
    let s1 = String::from("hello"); // tạo ra 1 biến s

    let len = calculate_length(&s1); //thêm syntax & trc biến sẽ là hành động reference (borrowing)
    //  lúc này hàm chỉ tạm thời mượn dùng xong sẽ trả thayy vì move và drop luôn biến
    println!("The length of '{s1}' is {len}.");
    //  vẫn có thể sử dụng được biến dù đã gọi hàm
}

fn calculate_length(s: &String) -> usize {
    s.len()
}