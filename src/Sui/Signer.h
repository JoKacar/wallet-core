// Copyright © 2017-2023 Trust Wallet.
//
// This file is part of Trust. The full Trust copyright notice, including
// terms governing use, modification, and redistribution, is contained in the
// file LICENSE at the root of the source code distribution tree.

#pragma once

#include "Data.h"
#include "PrivateKey.h"
#include "proto/Sui.pb.h"
#include "proto/TransactionCompiler.pb.h"

namespace TW::Sui {

/// Helper class that performs Sui transaction signing.
class Signer {
public:
    /// Hide default constructor
    Signer() = delete;

    /// Signs a Proto::SigningInput transaction
    static Proto::SigningOutput sign(const Proto::SigningInput& input);

    static TxCompiler::Proto::PreSigningOutput preImageHashes(const Proto::SigningInput& input);

    /// Get transaction data to be signed (with a type tag).
    static Data transactionPreimage(const Proto::SigningInput& input);

    static std::string signatureScheme(const Data& signature, const PublicKey& publicKey);
};

} // namespace TW::Sui