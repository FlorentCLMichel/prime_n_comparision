use std::env;
use std::io::{BufWriter, Write};
use std::time::Instant;

#[inline(always)]
fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    // use u64 to avoid overflow for i*i
    let n64 = n as u64;
    let mut i: u64 = 3;

    while i * i <= n64 {
        if n64 % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

#[inline(always)]
fn gen_primes<F>(limit: u32, mut callback: F)
where
    F: FnMut(u32, u32),
{
    let mut count = 0;

    // handle prime 2 separately
    if limit > 0 {
        count = 1;
        callback(1, 2);
    }

    let mut n: u32 = 3;

    while count < limit {
        if is_prime(n) {
            count += 1;
            callback(count, n);
        }
        n += 2; // skip evens
    }
}

fn main() {
    let mut args = env::args().skip(1);

    let n: u32 = args.next().unwrap().parse().unwrap();
    let buckets: u32 = args.next().unwrap().parse().unwrap();

    let file = std::fs::File::create("./benchmark_rust").unwrap();
    let mut writer = BufWriter::new(file);

    let start = Instant::now();
    let mut buffer = Vec::with_capacity((n as usize) * 24); // pre-alloc

    gen_primes(n, |count, prime| {
        if count % buckets == 0 || count == n {
            let _ = write!(
                &mut buffer,
                "{},{},{}\n",
                count,
                prime,
                start.elapsed().as_nanos()
            );
        }
    });

    writer.write_all(&buffer).unwrap();
    writer.flush().unwrap();
}
