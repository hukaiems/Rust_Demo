use std::time::Duration;

// trpl từ tokio crates thường được sử dụng cho async runtime

fn main() {
    trpl::run(async {
        // cho function đầu tiên trpl::run để có thể chạy async
        trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
                // sleep trong 500 milliseconds 
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    });
}