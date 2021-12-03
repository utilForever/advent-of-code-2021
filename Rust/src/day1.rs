pub fn generator(input: &str) -> Vec<u64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn part1(input: &[u64]) -> usize {
    let mut count = 0;

    for i in 1..input.len() {
        if input[i] > input[i - 1] {
            count += 1;
        }
    }

    count
}

pub fn part2(input: &[u64]) -> usize {
    let mut count = 0;

    for i in 0..input.len() - 3 {
        let sum_a = input[i] + input[i + 1] + input[i + 2];
        let sum_b = input[i + 1] + input[i + 2] + input[i + 3];

        if sum_b > sum_a {
            count += 1
        }
    }

    count
}
