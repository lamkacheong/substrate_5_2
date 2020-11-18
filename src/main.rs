fn sum(array: &[u32]) -> Option<u32>{
    let mut sum:Option<u32> = Some(0);
    for &i in array{
        sum = sum.unwrap().checked_add(i);
    };
    sum
}
fn main() {
    let a = [1,4294967295];
    let b = [1,2,3,4,5];
    println!("{:?}", sum(&a));
    println!("{:?}", sum(&b));
}