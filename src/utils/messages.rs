// `zks config` messages
pub const CONFIG_OVERRIDE_PROMPT_MSG: &str = "Config already exists. Do you want to overwrite it?";
pub const CONFIG_CREATE_PROMPT_MSG: &str = "This config does not exist. Do you want to create it?";
pub const CONFIG_EDIT_PROMPT_MSG: &str = "What config do you want to edit?";
pub const CONFIG_SET_PROMPT_MSG: &str = "What config do you want to set?";
pub const CONFIG_DELETE_PROMPT_MSG: &str = "Are you sure you want to delete this config?";
pub const CONFIG_SELECTION_TO_DELETE_PROMPT_MSG: &str = "What config do you want to delete?";
pub const CONFIG_TO_DISPLAY_PROMPT_MSG: &str = "What config do you want to see?";
pub const L1_RPC_URL_PROMPT_MSG: &str = "L1 RPC URL";
pub const L1_CHAIN_ID_PROMPT_MSG: &str = "L1 CHAIN ID";
pub const L2_RPC_URL_PROMPT_MSG: &str = "L2 RPC URL";
pub const L2_CHAIN_ID_PROMPT_MSG: &str = "L2 CHAIN ID";
pub const L2_EXPLORER_URL_PROMPT_MSG: &str = "L2 Explorer URL";
pub const L1_EXPLORER_URL_PROMPT_MSG: &str = "L1 Explorer URL";
pub const PRIVATE_KEY_PROMPT_MSG: &str = "Private key";
pub const ADDRESS_PROMPT_MSG: &str = "Address";
pub const CONTRACTS_GOVERNANCE_PROMPT_MSG: &str = "Governance contract address";
pub const CONTRACTS_GOVERNANCE_PRIVATE_KEY_PROMPT_MSG: &str = "Governance owner private key";
pub const CONTRACTS_BRIDGEHUB_ADMIN_PRIVATE_KEY_PROMPT_MSG: &str = "Bridgehub admin private key";
pub const CONTRACTS_BRIDGEHUB_OWNER_PRIVATE_KEY_PROMPT_MSG: &str = "Bridgehub owner private key";
pub const DATABASE_SERVER_URL_PROMPT_MSG: &str = "Server database URL";
pub const DATABASE_PROVER_URL_PROMPT_MSG: &str = "Prover database URL";

// `zks db` messages
pub const DATABASE_PROVER_RESTART_ALREADY_PROVED_BATCH_PROOF_CONFIRMATION_MSG: &str =
    "The batch proof is already sent to the server. Do you want to restart it anyways?";
pub const DATABASE_PROVER_RESTART_BATCH_PROOF_CONFIRMATION_MSG: &str =
    "You're about to delete unrecoverable data from the database. Are you sure you want to proceed?";
pub const DATABASE_PROVER_PROTOCOL_VERSION_PROMPT_MSG: &str = "Protocol version";
pub const DATABASE_PROVER_PROTOCOL_VERSION_PATCH_PROMPT_MSG: &str = "Protocol version patch";
pub const DATABASE_PROVER_RECURSION_SCHEDULER_VK_HASH_PROMPT_MSG: &str =
    "Recursion Scheduler Level VK Hash";
pub const DATABASE_PROVER_RECURSION_NODE_VK_HASH_PROMPT_MSG: &str = "Recursion Node Level VK Hash";
pub const DATABASE_PROVER_RECURSION_LEAF_VK_HASH_PROMPT_MSG: &str = "Recursion Leaf Level VK Hash";
pub const DATABASE_PROVER_RECURSION_CIRCUITS_SET_PROMPT_MSG: &str =
    "Recursion Circuits Set VKs Hash";
