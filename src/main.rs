mod day_four;
mod day_one;
mod day_three;
mod day_two;

mod utils;

fn main() {
    let a = "2-4,6-8";
    let intervals = a
        .split(",")
        .map(|x| {
            x.split("-")
                .map(|x| x.parse::<usize>().expect("Invalid interval"))
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();
    for interval in intervals {
        for i in interval {
            println!("{}", i)
        }
    }
    println!("Hello, world!");
}
