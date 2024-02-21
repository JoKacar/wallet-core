// SPDX-License-Identifier: Apache-2.0
//
// Copyright © 2017 Trust Wallet.

use crate::modules::message_builder::MessageBuilder;
use crate::modules::tx_signer::TxSigner;
use crate::SOLANA_ALPHABET;
use std::borrow::Cow;
use tw_coin_entry::coin_context::CoinContext;
use tw_coin_entry::error::{SigningError, SigningErrorType, SigningResult};
use tw_coin_entry::signing_output_error;
use tw_encoding::{base58, base64};
use tw_proto::Solana::Proto;

pub struct SolanaSigner;

impl SolanaSigner {
    pub fn sign(
        coin: &dyn CoinContext,
        input: Proto::SigningInput<'_>,
    ) -> Proto::SigningOutput<'static> {
        Self::sign_impl(coin, input)
            .unwrap_or_else(|e| signing_output_error!(Proto::SigningOutput, e))
    }

    fn sign_impl(
        _coin: &dyn CoinContext,
        input: Proto::SigningInput<'_>,
    ) -> SigningResult<Proto::SigningOutput<'static>> {
        let encode = move |data| match input.tx_encoding {
            Proto::Encoding::Base58 => base58::encode(data, &SOLANA_ALPHABET),
            Proto::Encoding::Base64 => base64::encode(data, false),
        };

        let builder = MessageBuilder::new(input);
        let signing_keys = builder.signing_keys()?;
        let unsigned_msg = builder.build()?;

        let encoded_unsigned = bincode::serialize(&unsigned_msg)
            .map_err(|_| SigningError(SigningErrorType::Error_internal))?;
        let encoded_unsigned = encode(&encoded_unsigned);

        let signed_tx = TxSigner::sign_versioned(unsigned_msg, &signing_keys)?;

        let encoded_tx = bincode::serialize(&signed_tx)
            .map_err(|_| SigningError(SigningErrorType::Error_internal))?;
        let encoded_tx = encode(&encoded_tx);

        Ok(Proto::SigningOutput {
            encoded: Cow::from(encoded_tx),
            unsigned_tx: Cow::from(encoded_unsigned),
            ..Proto::SigningOutput::default()
        })
    }
}