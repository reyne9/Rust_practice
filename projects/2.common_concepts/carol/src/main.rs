fn main() {
    // 1일차부터 12일차까지의 서수
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth",
    ];

    // 1일차부터 12일차까지의 선물 목록
    let gifts = [
        "partridge in a pear tree.",
        "Two turtle doves,",
        "Three French hens,",
        "Four calling birds,",
        "Five golden rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,",
    ];

    for day in 0..days.len() {
        println!(
            "On the {} day of Christmas, my true love sent to me",
            days[day]
        );

        // 현재 일차(day)부터 0번 인덱스까지 거꾸로 순회하며 선물 출력
        for gift in (0..=day).rev() {
            if gift == 0 && day == 0 {
                println!("A {}", gifts[gift]);
            } else if gift == 0 {
                println!("And a {}", gifts[gift]);
            } else {
                println!("{}", gifts[gift]);
            }
        }

        // 각 절 사이에 빈 줄 추가
        if day < days.len() - 1 {
            println!();
        }
    }
}
