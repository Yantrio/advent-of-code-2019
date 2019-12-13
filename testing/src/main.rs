fn main() {
    let layers = vec![vec![1, 0], vec![0, 1]];
    println!("{:?}", layers);

    let result = vec![0, 0];

    let test = layers.iter().fold(vec![0, 0], |acc, x| {
        x.iter()
            .enumerate()
            .map(|(idx, val)| acc[idx] + val)
            .collect::<Vec<i32>>()
    });
    println!("{:?}", test);
}
