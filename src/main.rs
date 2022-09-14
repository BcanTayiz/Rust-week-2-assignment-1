use blockchainlib::*;

fn main () {
    let difficulty = 0x000ffffffffffffffffffffffffffff;
    let mut block = Block::new(0,0,vec![0;32],0,"Genesis Block!".to_owned(),
    difficulty);

    block.hash = block.hash();

    println!("{:?}",&block);

    block.mine();

    println!("{:?}",&block);

    let mut last_hash = block.hash.clone();

    let mut blockchain = Blockchain{
        blocks: vec![block],
    };

    println!("Verify: {}", &blockchain.verify());

    return ;


}

