import { assert, test } from "vitest";

import { runScenario, pause, CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, AppBundleSource,  fakeActionHash, fakeAgentPubKey, fakeEntryHash } from '@holochain/client';
import { decode } from '@msgpack/msgpack';

import { createTokenGatedRoom } from './common.js';

test('create a TokenGatedRoom and get all lobbies', async () => {
  await runScenario(async scenario => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + '/../workdir/token-memproof.happ';

    // Set up the app to be installed 
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    // Bob gets all lobbies
    let collectionOutput: Record[] = await bob.cells[0].callZome({
      zome_name: "lobby",
      fn_name: "get_all_lobbies",
      payload: null
    });
    assert.equal(collectionOutput.length, 0);

    // Alice creates a TokenGatedRoom
    const createdRecord: Record = await createTokenGatedRoom(alice.cells[0]);
    assert.ok(createdRecord);
    
    await pause(1200);
    
    // Bob gets all lobbies again
    collectionOutput = await bob.cells[0].callZome({
      zome_name: "lobby",
      fn_name: "get_all_lobbies",
      payload: null
    });
    assert.equal(collectionOutput.length, 1);
    assert.deepEqual(createdRecord, collectionOutput[0]);    
  });
});

