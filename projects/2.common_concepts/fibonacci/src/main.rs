/*
제어흐름문 예제 : n번째 피보나치 수 생성하기

참고: 피보나치 수열이란 첫째, 둘째항이 1이고 이후의 항은 바로 앞 두 항의 합인 수열을 말한다.



*/
use std::io;

fn main() {
    println!("너는 피보나치 수열의 몇 번째 항을 원해?");

    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("오류:숫자를 다시 입력해줘");

    let n: u32 = n.trim().parse().expect("숫자를 다시 입력해줘");

    for i in 0..n {

        let mut fib = 0;
        if i == 0 || i == 1 {
            1;
        } else {
            let mut a = 1;
            let mut b = 1;
            for _ in 2..=i {
                let c = a + b;
                a = b;
                b = c;
            }
            fib = b;
    }

    println!("피보나치 수열의 {}번째 항은: {}이야", i, fib);
}


}
