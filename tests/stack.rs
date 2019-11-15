#![cfg_attr(feature = "nightly", feature(async_await, async_closure))]
#![warn(future_incompatible, rust_2018_compatibility, rust_2018_idioms, unused)]
#![warn(missing_docs, clippy::pedantic)]
#![cfg_attr(feature = "strict", deny(warnings))]

use genawaiter::{
    generator_mut,
    stack::{Co, Gen, Shelf},
};

use futures::{executor::{block_on_stream, block_on}, stream::{Stream, StreamExt},};

async fn odd_numbers_less_than_ten(co: Co<'_, i32>) {
    for n in (1..).step_by(2).take_while(|&n| n < 10) {
        println!("back to stack frame");
        co.yield_(n).await;
    }
}

// async fn stream1() -> &'static impl Stream {
//     generator_mut!(gen, odd_numbers_less_than_ten);
//     gen
// }

#[test]
fn test_basic_stack() {
    generator_mut!(gen, odd_numbers_less_than_ten);

    let xs: Vec<_> = gen.into_iter().collect();
    assert_eq!(xs, [1, 3, 5, 7, 9]);
}

#[test]
fn test_basic_stream() {
    

    generator_mut!(gen, odd_numbers_less_than_ten);
    
    block_on(async { println!("{:?}", gen.next().await) });
    block_on(async { println!("{:?}", gen.next().await) });

    // gen.take(3);
    
    // gen.for_each(|x| println!("{:?}", x));
    
    // block_on_stream(s);

    // assert_eq!(xs, [1, 3, 5, 7, 9]);
}

#[test]
fn test_shelf() {
    let mut shelf = Shelf::new();
    let gen = unsafe { Gen::new(&mut shelf, odd_numbers_less_than_ten) };

    let xs: Vec<_> = gen.into_iter().collect();
    assert_eq!(xs, [1, 3, 5, 7, 9]);
}
