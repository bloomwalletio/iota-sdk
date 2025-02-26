// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    CoinType,
    HD_WALLET_TYPE,
    initLogger,
    SecretManager,
    utf8ToHex,
    Utils,
} from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// In this example we will sign with Ed25519.
// Run with command:
// yarn run-example ./how_tos/sign_and_verify_ed25519/sign-ed25519.ts

const FOUNDRY_METADATA = {
    standard: 'IRC30',
    name: 'NativeToken',
    description: 'A native token',
    symbol: 'NT',
    decimals: 6,
    logoUrl: 'https://my.website/nativeToken.png',
};
const ACCOUNT_INDEX = 0;
const INTERNAL_ADDRESS = false;
const ADDRESS_INDEX = 0;

async function run() {
    initLogger();

    try {
        if (!process.env.STRONGHOLD_PASSWORD) {
            throw new Error(
                '.env stronghold password is undefined, see .env.example',
            );
        }
        if (!process.env.NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1) {
            throw new Error('.env mnemonic is undefined, see .env.example');
        }
        const secretManager = new SecretManager({
            stronghold: {
                password: process.env.STRONGHOLD_PASSWORD,
                snapshotPath: 'sign_ed25519.stronghold',
            },
        });

        // A mnemonic can be generated with `Utils.generateMnemonic()`.
        // Store the mnemonic in the Stronghold snapshot, this needs to be done only the first time.
        // The mnemonic can't be retrieved from the Stronghold file, so make a backup in a secure place!
        await secretManager.storeMnemonic(
            process.env.NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1,
        );

        const bip32Chain = [
            HD_WALLET_TYPE,
            CoinType.Shimmer,
            ACCOUNT_INDEX,
            INTERNAL_ADDRESS ? 1 : 0,
            ADDRESS_INDEX,
        ];
        const message = utf8ToHex(JSON.stringify(FOUNDRY_METADATA));
        const ed25519Signature = await secretManager.signEd25519(
            message,
            bip32Chain,
        );
        console.log(
            `Public key: ${ed25519Signature.publicKey}\nSignature: ${ed25519Signature.signature}`,
        );

        const bech32Address = Utils.hexPublicKeyToBech32Address(
            ed25519Signature.publicKey,
            'rms',
        );
        console.log('Address: ' + bech32Address);
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
