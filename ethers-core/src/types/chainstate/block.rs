// Taken from https://github.com/tomusdrw/rust-web3/blob/master/src/types/block.rs
use crate::types::{Address, Bloom, Bytes, H256, U256, U64};
use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[cfg(feature = "celo")]
///
pub struct Randomness {
    /// The committed randomness for that block
    pub committed: Bytes,
    /// The revealed randomness for that block
    pub revealed: Bytes,
}

/// The block type returned from RPC calls.
/// This is generic over a `TX` type which will be either the hash or the
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct Block<TX> {
    /// Hash of the block
    pub hash: Option<H256>,
    /// Hash of the parent
    #[serde(rename = "parentHash")]
    pub parent_hash: H256,
    /// Hash of the uncles
    #[cfg(not(feature = "celo"))]
    #[serde(rename = "sha3Uncles")]
    pub uncles_hash: H256,
    /// Miner/author's address.
    #[serde(rename = "miner")]
    pub author: Address,
    /// State root hash
    #[serde(rename = "stateRoot")]
    pub state_root: H256,
    /// Transactions root hash
    #[serde(rename = "transactionsRoot")]
    pub transactions_root: H256,
    /// Transactions receipts root hash
    #[serde(rename = "receiptsRoot")]
    pub receipts_root: H256,
    /// Block number. None if pending.
    pub number: Option<U64>,
    /// Gas Used
    #[serde(rename = "gasUsed")]
    pub gas_used: U256,
    /// Gas Limit
    #[cfg(not(feature = "celo"))]
    #[serde(rename = "gasLimit")]
    pub gas_limit: U256,
    /// Extra data
    #[serde(rename = "extraData")]
    pub extra_data: Bytes,
    /// Logs bloom
    #[serde(rename = "logsBloom")]
    pub logs_bloom: Option<Bloom>,
    /// Timestamp
    pub timestamp: U256,
    /// Difficulty
    #[cfg(not(feature = "celo"))]
    pub difficulty: U256,
    /// Total difficulty
    #[serde(rename = "totalDifficulty")]
    pub total_difficulty: Option<U256>,
    /// Seal fields
    #[serde(default, rename = "sealFields")]
    pub seal_fields: Vec<Bytes>,
    /// Uncles' hashes
    #[cfg(not(feature = "celo"))]
    pub uncles: Vec<H256>,
    /// Transactions
    pub transactions: Vec<TX>,
    /// Size in bytes
    pub size: Option<U256>,
    /// Mix Hash
    #[serde(rename = "mixHash")]
    #[cfg(not(feature = "celo"))]
    pub mix_hash: Option<H256>,
    /// Nonce
    #[cfg(not(feature = "celo"))]
    pub nonce: Option<U64>,

    #[cfg(feature = "celo")]
    #[cfg_attr(docsrs, doc(cfg(feature = "celo")))]
    /// The block's randomness
    pub randomness: Randomness,
}

#[derive(Copy, Clone, Debug, PartialEq)]
/// A Block Hash or Block Number
pub enum BlockId {
    // TODO: May want to expand this to include the requireCanonical field
    // https://github.com/ethereum/EIPs/blob/master/EIPS/eip-1898.md
    /// A block hash
    Hash(H256),
    /// A block number
    Number(BlockNumber),
}

impl From<u64> for BlockId {
    fn from(num: u64) -> Self {
        BlockNumber::Number(num.into()).into()
    }
}

impl From<U64> for BlockId {
    fn from(num: U64) -> Self {
        BlockNumber::Number(num).into()
    }
}

impl From<BlockNumber> for BlockId {
    fn from(num: BlockNumber) -> Self {
        BlockId::Number(num)
    }
}

impl From<H256> for BlockId {
    fn from(hash: H256) -> Self {
        BlockId::Hash(hash)
    }
}

impl Serialize for BlockId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            BlockId::Hash(ref x) => {
                let mut s = serializer.serialize_struct("BlockIdEip1898", 1)?;
                s.serialize_field("blockHash", &format!("{:?}", x))?;
                s.end()
            }
            BlockId::Number(ref num) => num.serialize(serializer),
        }
    }
}

/// A block Number (or tag - "latest", "earliest", "pending")
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BlockNumber {
    /// Latest block
    Latest,
    /// Earliest block (genesis)
    Earliest,
    /// Pending block (not yet part of the blockchain)
    Pending,
    /// Block by number from canon chain
    Number(U64),
}

impl<T: Into<U64>> From<T> for BlockNumber {
    fn from(num: T) -> Self {
        BlockNumber::Number(num.into())
    }
}

impl Serialize for BlockNumber {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            BlockNumber::Number(ref x) => serializer.serialize_str(&format!("0x{:x}", x)),
            BlockNumber::Latest => serializer.serialize_str("latest"),
            BlockNumber::Earliest => serializer.serialize_str("earliest"),
            BlockNumber::Pending => serializer.serialize_str("pending"),
        }
    }
}

#[cfg(test)]
#[cfg(not(feature = "celo"))]
mod tests {
    use super::*;
    use crate::types::{Transaction, TxHash};

    #[test]
    fn deserialize_blk_no_txs() {
        let block = r#"{"number":"0x3","hash":"0xda53da08ef6a3cbde84c33e51c04f68c3853b6a3731f10baa2324968eee63972","parentHash":"0x689c70c080ca22bc0e681694fa803c1aba16a69c8b6368fed5311d279eb9de90","mixHash":"0x0000000000000000000000000000000000000000000000000000000000000000","nonce":"0x0000000000000000","sha3Uncles":"0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347","logsBloom":"0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000","transactionsRoot":"0x7270c1c4440180f2bd5215809ee3d545df042b67329499e1ab97eb759d31610d","stateRoot":"0x29f32984517a7d25607da485b23cefabfd443751422ca7e603395e1de9bc8a4b","receiptsRoot":"0x056b23fbba480696b65fe5a59b8f2148a1299103c4f57df839233af2cf4ca2d2","miner":"0x0000000000000000000000000000000000000000","difficulty":"0x0","totalDifficulty":"0x0","extraData":"0x","size":"0x3e8","gasLimit":"0x6691b7","gasUsed":"0x5208","timestamp":"0x5ecedbb9","transactions":["0xc3c5f700243de37ae986082fd2af88d2a7c2752a0c0f7b9d6ac47c729d45e067"],"uncles":[]}"#;
        let _block: Block<TxHash> = serde_json::from_str(&block).unwrap();
    }

    #[test]
    fn deserialize_blk_with_txs() {
        let block = r#"{"number":"0x3","hash":"0xda53da08ef6a3cbde84c33e51c04f68c3853b6a3731f10baa2324968eee63972","parentHash":"0x689c70c080ca22bc0e681694fa803c1aba16a69c8b6368fed5311d279eb9de90","mixHash":"0x0000000000000000000000000000000000000000000000000000000000000000","nonce":"0x0000000000000000","sha3Uncles":"0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347","logsBloom":"0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000","transactionsRoot":"0x7270c1c4440180f2bd5215809ee3d545df042b67329499e1ab97eb759d31610d","stateRoot":"0x29f32984517a7d25607da485b23cefabfd443751422ca7e603395e1de9bc8a4b","receiptsRoot":"0x056b23fbba480696b65fe5a59b8f2148a1299103c4f57df839233af2cf4ca2d2","miner":"0x0000000000000000000000000000000000000000","difficulty":"0x0","totalDifficulty":"0x0","extraData":"0x","size":"0x3e8","gasLimit":"0x6691b7","gasUsed":"0x5208","timestamp":"0x5ecedbb9","transactions":[{"hash":"0xc3c5f700243de37ae986082fd2af88d2a7c2752a0c0f7b9d6ac47c729d45e067","nonce":"0x2","blockHash":"0xda53da08ef6a3cbde84c33e51c04f68c3853b6a3731f10baa2324968eee63972","blockNumber":"0x3","transactionIndex":"0x0","from":"0xfdcedc3bfca10ecb0890337fbdd1977aba84807a","to":"0xdca8ce283150ab773bcbeb8d38289bdb5661de1e","value":"0x0","gas":"0x15f90","gasPrice":"0x4a817c800","input":"0x","v":"0x25","r":"0x19f2694eb9113656dbea0b925e2e7ceb43df83e601c4116aee9c0dd99130be88","s":"0x73e5764b324a4f7679d890a198ba658ba1c8cd36983ff9797e10b1b89dbb448e"}],"uncles":[]}"#;
        let _block: Block<Transaction> = serde_json::from_str(&block).unwrap();
    }
}

#[cfg(test)]
#[cfg(feature = "celo")]
mod celo_tests {
    use super::*;
    use crate::types::Transaction;

    #[test]
    fn block() {
        let block = r#"{"extraData":"0xd983010000846765746889676f312e31332e3130856c696e7578000000000000f8b2c0c080b841cfa11585812ec794c4baa46178690971b3c72e367211d68a9ea318ff500c5aeb7099cafc965240e3b57cf7355341cf76bdca74530334658370d2df7b2e030ab200f582027db017810fa05b4f35927f968f6be1a61e322d4ace3563feb8a489690a91c031fda640c55c216f6712a7bdde994338a5610080f58203ffb093cd643f5154979952791ff714eb885df0f18012f2211fb8c29a8947130dc3adf4ecb48a3c4a142a0faa51e5c60b048180","gasUsed":"0xbef6","hash":"0x37ac2818e50e61f0566caea102ed98677f2552fa86fed53443315ed11fe0eaad","logsBloom":"0x00000800000200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000020000000100000000000000000000000000000000000000000000000000000000000000080000000000001020000400000000000000000000000000000000000000000000000000000000000080000000000000000000000000400000000000000000000000000000000000000100000040004000000000000800000000000000000084000000000000000000000000000000000020000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000","miner":"0xcda518f6b5a797c3ec45d37c65b83e0b0748edca","number":"0x1b4","parentHash":"0xa6b4775f600c2981f9142cbc1361db02c7ba8c185a1110537b255356876301a2","randomness":{"committed":"0x049e84c89f1aa0e3a770b2545b05a30eb814dae322e7247fd2bf27e6cacb1f51","revealed":"0x5a8826bf59a7ed1ee86a9d6464fa9c1fcece78ffa7cf32b11a03ad251ddcefe6"},"receiptsRoot":"0x1724dc3e7c2bfa03974c1deedf5ea20ad30b72e25f3c62fbb5fd06fc107068d7","size":"0x3a0","stateRoot":"0xc45fa03e69dccb54b4981d23d77328ab8906ddd7a0d8238b9c54ae1a14df4d1c","timestamp":"0x5e90166d","totalDifficulty":"0x1b5","transactions":[{"blockHash":"0x37ac2818e50e61f0566caea102ed98677f2552fa86fed53443315ed11fe0eaad","blockNumber":"0x1b4","from":"0x456f41406b32c45d59e539e4bba3d7898c3584da","gas":"0x1312d00","gasPrice":"0x174876e800","feeCurrency":null,"gatewayFeeRecipient":null,"gatewayFee":"0x0","hash":"0xf7b1b588b1fc03305f556805812273d80fb61fc0ba7f812de27189e95c5ecfc5","input":"0xed385274000000000000000000000000b9ff7ab50a2f0fd3e2fb2814b016ac90c91df98f03386ba30000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000be951906eba2aa800000","nonce":"0x147","to":"0xa12a699c641cc875a7ca57495861c79c33d293b4","transactionIndex":"0x0","value":"0x0","v":"0x15e08","r":"0x5787d040d09a34cb2b9ffcd096be7fe66aa6a3ed0632f182d1f3045640a9ef8b","s":"0x7897f58740f2a1c645826579106a620c306fc56381520ae2f28880bb284c4abd"}],"transactionsRoot":"0xbc8cb40b809914b9cd735b12e9b1802cf5d85de5223a22bbdb249a7e8b45ec93"}"#;
        let _block: Block<Transaction> = serde_json::from_str(&block).unwrap();
    }
}
