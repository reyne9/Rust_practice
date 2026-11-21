fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("현재 count는 {count}이다.");
        let mut remaining = 10;
        loop {
            println!("남은 수는 {remaining}이다.");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("최종 count는 {count}이다.");
}
