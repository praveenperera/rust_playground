#![feature(test)]
extern crate test;

use easybench::bench;
use itertools::Itertools;

fn main() {
    println!("join {}", bench(|| join()));
    println!("simple_iter {}", bench(|| simple_iter()));
    println!("iter {}", bench(|| iter()));
}

fn join() -> String {
    vec!["a", "b", "c", "d", "e", "f"].into_iter().join(" ")
}

fn simple_iter() -> String {
    let strs = vec!["a", "b", "c", "d", "e", "f"]
        .into_iter()
        .intersperse(" ")
        .collect::<Vec<&str>>();

    let mut string = String::with_capacity(strs.len());

    for str in strs {
        string.push_str(str)
    }

    string
}

fn iter() -> String {
    let strs = vec!["a", "b", "c", "d", "e", "f"].into_iter();

    let mut string = String::with_capacity(strs.len() * 2);

    for str in strs {
        string.push_str(str);
        string.push(' ')
    }

    string.pop();
    string
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_join(b: &mut Bencher) {
        b.iter(|| join());
    }

    #[bench]
    fn bench_simple_iter(b: &mut Bencher) {
        b.iter(|| simple_iter());
    }

    #[bench]
    fn bench_iter(b: &mut Bencher) {
        b.iter(|| iter());
    }
}
