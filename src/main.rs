fn main() {
    let start_a = std::env::args().nth(1)
        .unwrap_or("65".to_string())
        .parse()
        .unwrap();
    let start_b = std::env::args().nth(2)
        .unwrap_or("8921".to_string())
        .parse()
        .unwrap();

    let result = puzzle(Generator::new(16807, start_a),
                        Generator::new(48271, start_b), 40000000);

    println!("FirstHalf = {}", result);

    let result = check_iterators(Generator::new(16807, start_a).filter(|&a| a % 4 == 0),
                        Generator::new(48271, start_b).filter(|&a| a % 8 == 0),
                                 5000000);

    println!("SecondHalf = {}", result);
}

type InternalValue = u64;

struct Generator{
    factor: InternalValue,
    value: InternalValue
}

impl Generator {
    pub fn new(factor: InternalValue, value: InternalValue) -> Self {
        Self { factor, value }
    }
}


const MOD_VALUE: InternalValue = 2147483647;

impl Iterator for Generator {
    type Item = InternalValue;

    fn next(&mut self) -> Option<Self::Item> {
        self.value = (self.value * self.factor) % MOD_VALUE;
        Some(self.value)
    }
}

fn eq(pair: &(InternalValue, InternalValue)) -> bool {
    let &(a, b) = pair;
    (a & 0xFFFF) == (b & 0xFFFF)
}

fn check_iterators<I0, I1>(it0: I0, it1: I1, count: usize) -> usize
    where I0: Iterator<Item=InternalValue>,
          I1: Iterator<Item=InternalValue>
{
    it0.zip(it1)
        .take(count)
        .filter(eq)
        .count()

}

fn puzzle(a: Generator, b: Generator, count: usize) -> usize {
    check_iterators(a, b, count)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generator() {
        let a = Generator::new(16807, 65);

        assert_eq!(vec![1092455, 1181022009, 245556042, 1744312007, 1352636452],
                   a.take(5).collect::<Vec<_>>());
    }

    #[test]
    fn test_puzzle() {
        let a = Generator::new(16807, 65);
        let b = Generator::new(48271, 8921);

        assert_eq!(588, puzzle(a, b, 40000000))
    }
}
