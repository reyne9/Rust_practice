/* 
화씨온도와 섭씨 온도 사이의 변환을 수행하는 프로그램을 작성하시오.
화씨온도(Fahrenheit)와 섭씨온도(Celsius) 사이의 변환 공식은 다음과 같습니다:
- 섭씨온도(C)에서 화씨온도(F)로 변환:
  F = (C * 9/5) + 32
- 화씨온도(F)에서 섭씨온도(C)로 변환:
  C = (F - 32) * 5/9
사용자로부터 온도와 단위를 입력받아 변환된 온도를 출력하는 프로그램을 작성하시오.
*/

use std::io;

fn main() {
  println!("온도를 입력하세요 (예: 100C 또는 212F):");
  let mut input = String::new();

  io::stdin()
      .read_line(&mut input)
      .expect("입력 읽기 실패");

    let input = input.trim();

    if input.ends_with('C') {
      let celsius: f64 = input.trim_end_matches('C').trim().parse().expect("온도를 다시 입력해주세요.");

      let result = (celsius * 9.0 / 5.0) + 32.0;
      
      println!("화씨 온도: {}F", result);


    } else if input.ends_with('F') {
      let fahrenheit: f64 = input.trim_end_matches('F').trim().parse().expect("온도를 다시 입력해주세요.");

      let result = (fahrenheit - 32.0) * 5.0 / 9.0;
      println!("섭씨 온도: {}C", result);

        
    } else {
        println!("잘못된 입력입니다. C 또는 F로 끝나는 온도를 입력해주세요.");
    }
    
}
