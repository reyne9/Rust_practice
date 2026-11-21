fn main() {
    print_labeled_measurement(5, 'h');
 //   another_function(5);
}

fn another_function(x: i32) {
    println!("x의 값은 {x}입니다.");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("측정값은 {value}{unit_label}입니다.");
}
