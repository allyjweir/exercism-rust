pub fn nth(n: u32) -> Option<u32> {
    let mut primes = [];
    let mut current_test = 2;
    while primes.len() <= n {
        let test_numbers = 1..current_test;
        for num in test_numbers {
           if current_test/num == 0 {
               current_test++;
               break;
           }
        }
        primes += current_test;
        current_test++;
    }
    Some(primes[-1])
}
