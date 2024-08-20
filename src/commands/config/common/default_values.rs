use zksync_ethers_rs::types::H160;

pub const DEFAULT_L1_RPC_URL: &str = "http://localhost:8545";
pub const DEFAULT_L2_RPC_URL: &str = "http://localhost:3050";
pub const DEFAULT_L2_EXPLORER_URL: &str = "http://localhost:3010";
pub const DEFAULT_L1_EXPLORER_URL: &str = "";
pub const DEFAULT_PRIVATE_KEY: &str =
    "0x7726827caac94a7f9e1b160f7ea819f172f7b6f9d2a97f992c38edeab82d4110";
// 0x36615Cf349d7F6344891B1e7CA7C72883F5dc049
pub const DEFAULT_ADDRESS: H160 = H160([
    0x36, 0x61, 0x5C, 0xf3, 0x49, 0xd7, 0xf6, 0x34, 0x48, 0x91, 0xb1, 0xe7, 0xca, 0x7c, 0x72, 0x88,
    0x3f, 0x5d, 0xc0, 0x48,
]);
// 0x5E6D086F5eC079ADFF4FB3774CDf3e8D6a34F7E9
pub const DEFAULT_CONTRACT_ADDRESS: H160 = H160([
    0x5E, 0x6D, 0x08, 0x6F, 0x5e, 0xC0, 0x79, 0xAD, 0xFF, 0x4F, 0xB3, 0x77, 0x4C, 0xdf, 0x3e, 0x8D,
    0x6a, 0x34, 0xF7, 0xE9,
]);