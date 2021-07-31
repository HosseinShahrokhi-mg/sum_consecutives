/// Just to make code nicer
type Number = i32;
type Numbers = Vec<Number>;
fn main() {
    let input = vec![1, 4, 4, 4, 0, 4, 3, 3, 1];

    println!("{:?}",sum_consecutives(&input));
}


/// Sums the numbers that are the same and consecutive.
fn sum_consecutives(numbers: &Numbers) -> Numbers {
    let mut out_vec : Numbers = Vec::new();
    let mut sum: Number;
    for i in numbers{
        sum = *i;
        
        if *i == numbers[*i+1.unwrap]{
            sum +=i;
            println!("i= {}:{}",i,sum);
        }
       out_vec.push(sum);
    }
    out_vec
}