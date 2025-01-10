// Mainnet 8333
// Testnet 18333
pub const PORT_MAINNET: u16 = 8333;
pub const PORT_TESTNET: u16 = 18333;

// Mainnet 0xD9B4BEF9
// Testnet 0x0709110B
//  bitcoin::network::constants::Network::Bitcoin.magic()
pub const MAGIC_MAINNET: u32 = 0xD9B4BEF9;
//  bitcoin::network::constants::Network::Testnet.magic()
pub const MAGIC_TESTNET: u32 = 0x0709110B;

pub const DNS_LIST_MAINNET: &'static [&'static str] = &[
    "seed.bitcoin.sipa.be.",
    "dnsseed.bluematt.me.",
    "dnsseed.bitcoin.dashjr.org.",
    "seed.bitcoinstats.com.",
    "seed.btc.petertodd.org.",
    "seed.bitcoin.sprovoost.nl.",
    "dnsseed.emzy.de.",
    "seed.bitcoin.wiz.biz.",
    // "seed.bitcoin.jonasschnelli.ch.", - producing unexpected output, sometimes works, other times gives error: failed to lookup address information: Name or service not known
];

pub const DNS_LIST_TESTNET: &'static [&'static str] = &[
    "testnet-seed.bitcoin.jonasschnelli.ch.",
    "seed.testnet.bitcoin.sprovoost.nl.",
    "testnet-seed.bluematt.me.",
    // "seed.tbtc.petertodd.org.", - gives error: failed to lookup address information: Name or service not known
];
