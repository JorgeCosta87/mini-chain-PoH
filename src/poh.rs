use sha2::{Sha256, Digest};
use std::cmp::min;

// https://github.com/solana-labs/solana/blob/master/sdk/program/src/clock.rs
// https://github.com/solana-labs/solana/blob/master/sdk/src/signature.rs
// https://github.com/solana-labs/solana/blob/7700cb3128c1f19820de67b81aa45d18f73d2ac0/poh/src/poh_service.rs#L9
// https://github.com/solana-labs/solana/blob/master/entry/src/poh.rs#L10
// https://github.com/solana-labs/solana/blob/7700cb3128c1f19820de67b81aa45d18f73d2ac0/poh/src/poh_service.rs#L20
// https://github.com/solana-foundation/specs
// https://www.linkedin.com/pulse/solanas-proof-of-historys-detailed-explanation-emil-kubanychbek-q49ne

pub type Hash = [u8; 32];

struct Entry {
    num_hashes: u32,
    hash: [u8; 32],
    mixin: Option<[u8;32]>,
    is_tick: bool
}

#[derive(Debug, PartialEq)]
pub enum PoHResult {
    Continue,
    TickComplete,
    SlotComplete,
}

#[derive(Debug)]
pub struct PoH {
    hash: Hash,
    hash_count: u64,
    hashes_per_tick: u64,
    ticks_per_slot: u64,
    tick_count: u64,
    slot: u64
}

impl PoH {
    pub fn new(seed: Hash, hashes_per_tick: u64, ticks_per_slot: u64) -> Self{
        PoH {
            hash: seed,
            hash_count: 0,
            hashes_per_tick,
            ticks_per_slot,
            tick_count:0,
            slot:0     
        }
    }

    fn compute_hash(&mut self, mixin_hash: Option<Hash>)
    {
        let mut hasher = Sha256::new();

        hasher.update(&self.hash);

        if let Some(hash) = mixin_hash {
            hasher.update(&hash);
        }
       
        self.hash = hasher.finalize().into();
        self.hash_count += 1;
    }

    pub fn compute_tick_hashes(&mut self) -> PoHResult 
    {
        let reamining_hashes = min(self.hashes_per_tick, self.hashes_per_tick - self.hash_count); // just for safety
        for _ in 0..reamining_hashes {
            self.compute_hash(None)
        }

        if self.hash_count >= self.hashes_per_tick {
            self.hash_count = 0;
            self. tick_count += 1;

            if self.tick_count == self.ticks_per_slot {
                self.tick_count = 0;
                self.slot += 1;
                
                return PoHResult::SlotComplete
            }
            return PoHResult::TickComplete
        }

        PoHResult::Continue
    }

    pub fn mixin_data(&mut self, mixin_hash: Hash){
        self.compute_hash(Some(mixin_hash));
    }

}
