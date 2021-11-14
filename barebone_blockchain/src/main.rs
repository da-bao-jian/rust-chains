use blockchainlib::*;
fn main() {
    let block = Block::new(
        0,
        0,
        vec![0; 32],
        0,
        0,
    );

    // println!("{:?}", &block);

    let h = block.hashing();

    println!("{:?}", &h);
}
