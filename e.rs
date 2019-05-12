#![feature(main)]
#[cfg(all(feature = "functional", not(feature = "future")))]
#[main]
fn e() {
    std::iter::repeat('e').for_each(|e| print!("{}", e));
}

#[cfg(all(not(feature = "functional"), feature = "future"))]
#[main]
fn e() {
    use tokio::prelude::stream::Stream;
    tokio::run(tokio::prelude::stream::iter_ok::<_, ()>(std::iter::repeat('e')).for_each(|e| { print!("{}", e); Ok(()) }));
}

#[cfg(all(not(feature = "functional"), not(feature = "future")))]
#[main]
fn e() -> ! {
    loop {
        print!("e");
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn teeeeest() {
    e();
    unreachable!("eeeeeeeeee?????");
  }
}

