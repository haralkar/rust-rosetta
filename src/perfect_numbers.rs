// Implements http://rosettacode.org/wiki/Perfect_numbers
#![feature(core)]

fn perfect_number(n: usize) -> bool {
  (1..(n / 2)+1).filter(|&i| n % i == 0).sum::<usize>() == n
}

#[cfg(not(test))]
fn main() {
  for n in (2..10_000).filter(|&n| perfect_number(n)) {
    println!("{}", n);
  }
}

#[test]
fn test_first_four() {
  let nums = (2..10_000).filter(|&n| perfect_number(n))
                              .collect::<Vec<usize>>();
  assert_eq!(nums, [6, 28, 496, 8128]);
}

#[test]
fn test_high_number() {
  assert!( perfect_number(33550336) );
}
