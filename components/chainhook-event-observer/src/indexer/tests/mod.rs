pub mod helpers;
use crate::utils::{AbstractBlock, Context};

use self::helpers::BlockEvent;
use super::{fork_scratch_pad::ForkScratchPad, StacksBlockPool};
use chainhook_types::{BitcoinBlockData, BlockchainEvent, StacksChainEvent};

pub type StacksChainEventExpectation = Box<dyn Fn(Option<StacksChainEvent>) -> ()>;

pub fn process_stacks_blocks_and_check_expectations(
    steps: Vec<(BlockEvent, StacksChainEventExpectation)>,
) {
    let mut blocks_processor = StacksBlockPool::new();
    for (block_event, check_chain_event_expectations) in steps.into_iter() {
        match block_event {
            BlockEvent::Block(block) => {
                let chain_event = blocks_processor
                    .process_block(block, &Context::empty())
                    .unwrap();
                check_chain_event_expectations(chain_event);
            }
            BlockEvent::Microblock(microblock) => {
                let chain_event = blocks_processor
                    .process_microblocks(vec![microblock], &Context::empty())
                    .unwrap();
                check_chain_event_expectations(chain_event);
            }
        }
    }
}

pub type BlockchainEventExpectation = Box<dyn Fn(Option<BlockchainEvent>) -> ()>;

pub fn process_bitcoin_blocks_and_check_expectations(
    steps: Vec<(BitcoinBlockData, BlockchainEventExpectation)>,
) {
    let mut blocks_processor = ForkScratchPad::new();
    for (block, check_chain_event_expectations) in steps.into_iter() {
        let chain_event = blocks_processor
            .process_header(block.get_header(), &Context::empty())
            .unwrap();
        check_chain_event_expectations(chain_event);
    }
}
