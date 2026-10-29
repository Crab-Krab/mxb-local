import { test } from 'node:test';
import assert from 'node:assert/strict';
import { finishSetup } from '../src/lib/finishSetup.mjs';

for (const modsPath of ['', 'C:\\Games\\MX Bikes']) {
  test(`app-only setup opens dashboard with ${modsPath ? 'manual' : 'detected'} folders`, async () => {
    let saved;
    let dashboard = false;
    await finishSetup({
      saveFolders: async () => { saved = { modsPath: modsPath || 'C:\\Detected\\MX Bikes', setupComplete: false }; },
      completeSetup: async () => { assert.ok(saved.modsPath); saved.setupComplete = true; },
      onComplete: async () => { dashboard = !!saved.modsPath && saved.setupComplete; },
    });
    assert.equal(dashboard, true, 'reloaded config must satisfy the dashboard readiness condition');
  });
}

test('dashboard waits for completion to persist', async () => {
  let release;
  let dashboard = false;
  const pending = new Promise(resolve => { release = resolve; });
  const finished = finishSetup({
    saveFolders: async () => {}, completeSetup: () => pending,
    onComplete: () => { dashboard = true; },
  });
  await new Promise(resolve => setImmediate(resolve));
  assert.equal(dashboard, false);
  release();
  await finished;
  assert.equal(dashboard, true);
});

for (const failing of ['saveFolders', 'completeSetup']) {
  test(`${failing} failure stays available to the setup error handler`, async () => {
    const calls = [];
    const actions = Object.fromEntries(['saveFolders', 'completeSetup', 'onComplete'].map(name => [name, async () => {
      calls.push(name);
      if (name === failing) throw new Error('disk write failed');
    }]));
    await assert.rejects(finishSetup(actions), /disk write failed/);
    assert.equal(calls.includes('onComplete'), false);
    assert.deepEqual(calls, failing === 'saveFolders' ? ['saveFolders'] : ['saveFolders', 'completeSetup']);
  });
}

test('an integration choice still waits for its own final consent step', async () => {
  const calls = [];
  await finishSetup(Object.fromEntries(['saveFolders', 'completeSetup', 'onComplete', 'onIntegration'].map(name => [name, async () => { calls.push(name); }])));
  assert.deepEqual(calls, ['saveFolders', 'onIntegration']);
});
