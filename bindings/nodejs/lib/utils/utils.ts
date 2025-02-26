// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { plainToInstance } from 'class-transformer';
import { callUtilsMethod } from '../bindings';
import {
    Address,
    Ed25519Address,
    BlockId,
    HexEncodedString,
    Block,
    Ed25519Signature,
    TransactionEssence,
    Response,
    MilestonePayload,
    MilestoneId,
    TransactionPayload,
    TransactionId,
} from '../types';

/** Utils class for utils. */
export class Utils {
    /**
     * Generates a new mnemonic.
     */
    static generateMnemonic(): string {
        return callUtilsMethod({
            name: 'generateMnemonic',
        });
    }

    /**
     * Returns a hex encoded seed for a mnemonic.
     */
    static mnemonicToHexSeed(mnemonic: string): HexEncodedString {
        return callUtilsMethod({
            name: 'mnemonicToHexSeed',
            data: {
                mnemonic,
            },
        });
    }

    /**
     * Computes the alias id for the given alias output id.
     */
    static computeAliasId(outputId: string): string {
        return callUtilsMethod({
            name: 'computeAliasId',
            data: {
                outputId,
            },
        });
    }

    /**
     * Computes the NFT id for the given NFT output id.
     */
    static computeNftId(outputId: string): string {
        return callUtilsMethod({
            name: 'computeNftId',
            data: {
                outputId,
            },
        });
    }

    /**
     * Computes the foundry id.
     */
    static computeFoundryId(
        aliasAddress: string,
        serialNumber: number,
        tokenSchemeKind: number,
    ): string {
        return callUtilsMethod({
            name: 'computeFoundryId',
            data: {
                aliasAddress,
                serialNumber,
                tokenSchemeKind,
            },
        });
    }

    /**
     * Returns a valid Address parsed from a String.
     */
    static parseBech32Address(address: string): Address {
        const addr = callUtilsMethod({
            name: 'parseBech32Address',
            data: {
                address,
            },
        });

        const parsed = JSON.parse(addr) as Response<Ed25519Address>;
        return plainToInstance(Ed25519Address, parsed.payload);
    }

    /**
     * Returns a block ID (Blake2b256 hash of the block bytes)
     */
    static blockId(block: Block): BlockId {
        return callUtilsMethod({
            name: 'blockId',
            data: {
                block,
            },
        });
    }

    /**
     * Returns a Milestone ID (Blake2b256 hash of the milestone essence)
     */
    static milestoneId(payload: MilestonePayload): MilestoneId {
        return callUtilsMethod({
            name: 'milestoneId',
            data: {
                payload,
            },
        });
    }

    /**
     * Returns the transaction ID (Blake2b256 hash of the provided transaction payload)
     */
    static transactionId(payload: TransactionPayload): TransactionId {
        return callUtilsMethod({
            name: 'transactionId',
            data: {
                payload,
            },
        });
    }

    /**
     * Transforms bech32 to hex.
     */
    static bech32ToHex(bech32: string): string {
        return callUtilsMethod({
            name: 'bech32ToHex',
            data: {
                bech32,
            },
        });
    }

    /**
     * Transforms a hex encoded address to a bech32 encoded address.
     */
    static hexToBech32(hex: string, bech32Hrp: string): string {
        return callUtilsMethod({
            name: 'hexToBech32',
            data: {
                hex,
                bech32Hrp,
            },
        });
    }

    /**
     * Transforms an alias id to a bech32 encoded address.
     */
    static aliasIdToBech32(aliasId: string, bech32Hrp: string): string {
        return callUtilsMethod({
            name: 'aliasIdToBech32',
            data: {
                aliasId,
                bech32Hrp,
            },
        });
    }

    /**
     * Transforms an nft id to a bech32 encoded address.
     */
    static nftIdToBech32(nftId: string, bech32Hrp: string): string {
        return callUtilsMethod({
            name: 'nftIdToBech32',
            data: {
                nftId,
                bech32Hrp,
            },
        });
    }

    /**
     * Transforms a hex encoded public key to a bech32 encoded address.
     */
    static hexPublicKeyToBech32Address(hex: string, bech32Hrp: string): string {
        return callUtilsMethod({
            name: 'hexPublicKeyToBech32Address',
            data: {
                hex,
                bech32Hrp,
            },
        });
    }

    /**
     * Checks if a String is a valid bech32 encoded address.
     */
    static isAddressValid(address: string): boolean {
        return callUtilsMethod({
            name: 'isAddressValid',
            data: {
                address,
            },
        });
    }

    /**
     * Compute the hash of a transaction essence.
     */
    static hashTransactionEssence(
        essence: TransactionEssence,
    ): HexEncodedString {
        return callUtilsMethod({
            name: 'hashTransactionEssence',
            data: {
                essence,
            },
        });
    }

    /**
     * Verifies an ed25519 signature against a message.
     */
    static verifyEd25519Signature(
        signature: Ed25519Signature,
        message: HexEncodedString,
    ): boolean {
        return callUtilsMethod({
            name: 'verifyEd25519Signature',
            data: {
                signature,
                message,
            },
        });
    }

    /**
     * Verifies a Secp256k1Ecdsa signature against a message.
     */
    static verifySecp256k1EcdsaSignature(
        publicKey: HexEncodedString,
        signature: HexEncodedString,
        message: HexEncodedString,
    ): boolean {
        return callUtilsMethod({
            name: 'verifySecp256k1EcdsaSignature',
            data: {
                publicKey,
                signature,
                message,
            },
        });
    }

    /**
     * Verify if a mnemonic is a valid BIP39 mnemonic.
     */
    static verifyMnemonic(mnemonic: string): void {
        return callUtilsMethod({
            name: 'verifyMnemonic',
            data: { mnemonic },
        });
    }
}
