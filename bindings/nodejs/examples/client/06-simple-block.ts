// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Client, initLogger } from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./client/06-simple-block.ts

// In this example we will send a block without a payload.
async function run() {
    initLogger();
    if (!process.env.NODE_URL) {
        throw new Error('.env NODE_URL is undefined, see .env.example');
    }

    const client = new Client({
        // Insert your node URL in the .env.
        nodes: [process.env.NODE_URL],
        localPow: true,
    });

    try {
        // Create block with no payload
        const blockIdAndBlock = await client.buildAndPostBlock();
        console.log('Block:', blockIdAndBlock, '\n');

        console.log(
            `Empty block sent: ${process.env.EXPLORER_URL}/block/${blockIdAndBlock[0]}`,
        );
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
