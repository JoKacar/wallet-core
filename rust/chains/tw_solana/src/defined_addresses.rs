// SPDX-License-Identifier: Apache-2.0
//
// Copyright © 2017 Trust Wallet.

use crate::address::SolanaAddress;
use lazy_static::lazy_static;

macro_rules! define {
    ($name:ident = $s:literal) => {
        lazy_static! {
            pub static ref $name: SolanaAddress = SolanaAddress::from($s);
        }
    };
}

define!(SYSTEM_PROGRAM_ID_ADDRESS = "11111111111111111111111111111111");
define!(STAKE_PROGRAM_ID_ADDRESS = "Stake11111111111111111111111111111111111111");
define!(TOKEN_PROGRAM_ID_ADDRESS = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
define!(ASSOCIATED_TOKEN_PROGRAM_ID_ADDRESS = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");
define!(SYSVAR_RENT_ID_ADDRESS = "SysvarRent111111111111111111111111111111111");
define!(SYSVAR_CLOCK_ID_ADDRESS = "SysvarC1ock11111111111111111111111111111111");
define!(STAKE_CONFIG_ID_ADDRESS = "StakeConfig11111111111111111111111111111111");
define!(NULL_ID_ADDRESS = "11111111111111111111111111111111");
define!(SYSVAR_STAKE_HISTORY_ID_ADDRESS = "SysvarStakeHistory1111111111111111111111111");
define!(MEMO_PROGRAM_ID_ADDRESS = "MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr");
define!(SYSVAR_RECENT_BLOCKHASHS_ADDRESS = "SysvarRecentB1ockHashes11111111111111111111");
