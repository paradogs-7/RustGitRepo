fn main() {

    let mut input = String::new();

    std::io::stdin().read_line(&mut input).expect("Crashed");

    let n: usize= input.trim().parse().expect("Craashed.2");

    input.clear();
    std::io::stdin().read_line(&mut input).expect("Crashed.3");

    let numbers: Vec<usize> = input.split_whitespace().map(|s| s.parse::<usize>().expect("Crashed.4")).collect();

    let mut sorted = numbers.clone();
    sorted.sort_by(|a,b| b.cmp(a));

    println!("{}", sorted[0]*sorted[1]);

}






// let numbers: Vec<i32> = input
//         .split_whitespace()
//         .map(|s| s.parse::<i32>().unwrap())
//         .collect();

//     let mut sorted = numbers.clone();
//     sorted.sort_by(|a, b| b.cmp(a));