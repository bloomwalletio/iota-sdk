// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::{
    client::{hex_public_key_to_bech32_address, hex_to_bech32, verify_mnemonic, Client},
    types::block::{
        address::{dto::AddressDto, Address, ToBech32Ext},
        output::{AliasId, FoundryId, NftId},
        payload::{transaction::TransactionEssence, MilestonePayload, TransactionPayload},
        signature::Ed25519Signature,
        Block,
    },
};
use zeroize::Zeroize;

use crate::{method::UtilsMethod, response::Response, Result};

/// Call a utils method.
pub(crate) fn call_utils_method_internal(method: UtilsMethod) -> Result<Response> {
    let response = match method {
        UtilsMethod::Bech32ToHex { bech32 } => Response::Bech32ToHex(Client::bech32_to_hex(bech32)?),
        UtilsMethod::HexToBech32 { hex, bech32_hrp } => Response::Bech32Address(hex_to_bech32(&hex, bech32_hrp)?),
        UtilsMethod::AliasIdToBech32 { alias_id, bech32_hrp } => {
            Response::Bech32Address(alias_id.to_bech32(bech32_hrp))
        }
        UtilsMethod::NftIdToBech32 { nft_id, bech32_hrp } => Response::Bech32Address(nft_id.to_bech32(bech32_hrp)),
        UtilsMethod::HexPublicKeyToBech32Address { hex, bech32_hrp } => {
            Response::Bech32Address(hex_public_key_to_bech32_address(&hex, bech32_hrp)?)
        }
        UtilsMethod::ParseBech32Address { address } => Response::ParsedBech32Address(AddressDto::from(address.inner())),
        UtilsMethod::IsAddressValid { address } => Response::Bool(Address::is_valid_bech32(&address)),
        UtilsMethod::GenerateMnemonic => Response::GeneratedMnemonic(Client::generate_mnemonic()?),
        UtilsMethod::MnemonicToHexSeed { mut mnemonic } => {
            let response = Response::MnemonicHexSeed(Client::mnemonic_to_hex_seed(&mnemonic)?);
            mnemonic.zeroize();
            response
        }
        UtilsMethod::BlockId { block } => {
            let block = Block::try_from_dto_unverified(block)?;
            Response::BlockId(block.id())
        }
        UtilsMethod::MilestoneId { payload } => {
            let payload = MilestonePayload::try_from_dto_unverified(payload)?;
            Response::MilestoneId(payload.id())
        }
        UtilsMethod::TransactionId { payload } => {
            let payload = TransactionPayload::try_from_dto_unverified(payload)?;
            Response::TransactionId(payload.id())
        }
        UtilsMethod::ComputeAliasId { output_id } => Response::AliasId(AliasId::from(&output_id)),
        UtilsMethod::ComputeNftId { output_id } => Response::NftId(NftId::from(&output_id)),
        UtilsMethod::ComputeFoundryId {
            alias_address,
            serial_number,
            token_scheme_kind,
        } => Response::FoundryId(FoundryId::build(&alias_address, serial_number, token_scheme_kind)),
        UtilsMethod::HashTransactionEssence { essence } => Response::TransactionEssenceHash(prefix_hex::encode(
            TransactionEssence::try_from_dto_unverified(essence)?.hash(),
        )),
        UtilsMethod::VerifyMnemonic { mut mnemonic } => {
            verify_mnemonic(&mnemonic)?;
            mnemonic.zeroize();
            Response::Ok
        }
        UtilsMethod::VerifyEd25519Signature { signature, message } => {
            let signature = Ed25519Signature::try_from(signature)?;
            let message: Vec<u8> = prefix_hex::decode(message)?;
            Response::Bool(signature.verify(&message)?)
        }
        UtilsMethod::VerifySecp256k1EcdsaSignature {
            public_key,
            signature,
            message,
        } => {
            use crypto::signatures::secp256k1_ecdsa;
            use iota_sdk::types::block::Error;
            let public_key = prefix_hex::decode(public_key)?;
            let public_key = secp256k1_ecdsa::PublicKey::try_from_bytes(&public_key).map_err(Error::from)?;
            let signature = prefix_hex::decode(signature)?;
            let signature = secp256k1_ecdsa::Signature::try_from_bytes(&signature).map_err(Error::from)?;
            let message: Vec<u8> = prefix_hex::decode(message)?;
            Response::Bool(public_key.verify(&signature, &message))
        }
    };
    Ok(response)
}
