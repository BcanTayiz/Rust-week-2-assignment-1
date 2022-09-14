
use super::*;

pub enum BlockValidationErr {
    MismatchedIndex,
    InvalidHash,
    AchronologicalTimestamp,
    MismatchedPreviousHash,
    InvalidGenesisBlockFormat,
    InvalidInput,
    InsufficientInputValue,
    InvalidCoinbaseTransaction,
}

pub struct Blockchain{
    pub blocks: Vec<Block>,
}

impl Blockchain{
    pub fn update_with_block(&mut self, block:Block) -> Result<(),BlockValidationErr> {
        let i = self.blocks.len();
            if block.index != i as u32 {
                return Err(BlockValidationErr::MismatchedIndex);
            } else if !block::check_difficulty(&block.hash(),block.difficulty){
                return Err(BlockValidationErr::InvalidHash);
            } else if i != 0{
                // Not genesis block
                let prev_block = &self.blocks[i-1];
                if block.timestamp <= prev_block.timestamp{
                    return Err(BlockValidationErr::AchronologicalTimestamp)
                } else if block.prev_block_hash != prev_block.hash{
                    return Err(BlockValidationErr::MismatchedPreviousHash)
                }
            } else {
                //Genesis block
                if block.prev_block_hash != vec![0;32]{
                    return Err(BlockValidationErr::InvalidGenesisBlockFormat)
                }
            }

            Ok(())
        }

}