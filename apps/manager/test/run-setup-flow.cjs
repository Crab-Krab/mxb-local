const { chromium } = require('playwright');
const { build } = require('esbuild');
const fs = require('node:fs/promises');
const os = require('node:os');
const path = require('node:path');
const http = require('node:http');
const assert = require('node:assert/strict');

(async () => {
  const root = path.resolve(__dirname, '..');
  const tempParent = await fs.realpath(os.tmpdir());
  const output = await fs.mkdtemp(path.join(tempParent, 'mxb-setup-flow-'));
  let browser, server;
  try {
    await build({
      entryPoints: [path.join(__dirname, 'setup-flow-fixture.tsx')],
      outfile: path.join(output, 'fixture.js'), bundle: true, platform: 'browser',
      format: 'iife', jsx: 'automatic', tsconfig: path.join(root, 'tsconfig.json'),
      define: { 'process.env.NODE_ENV': '"test"' },
      plugins: [{ name: 'native-api-fixtures', setup(bundler) {
        bundler.onResolve({ filter: /^(?:@frost\/shared\/api\/mods|@frost\/shared\/lib\/usePlatform|@tauri-apps\/plugin-dialog|@\/i18n|@\/Context\/FrostmodContext)$|Mxbmrp3Suggestion$/ },
          () => ({ path: path.join(__dirname, 'setup-flow-mocks.tsx') }));
      } }],
    });
    server = http.createServer(async (req, res) => {
      const pathname = new URL(req.url, 'http://localhost').pathname;
      if (pathname === '/') {
        res.setHeader('Content-Type', 'text/html');
        res.end('<!doctype html><html><body><div id="root"></div><script src="/fixture.js"></script></body></html>');
      } else if (pathname === '/fixture.js') {
        res.setHeader('Content-Type', 'text/javascript');
        res.end(await fs.readFile(path.join(output, 'fixture.js')));
      } else res.writeHead(404).end();
    });
    await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
    browser = await chromium.launch({ headless: true });
    const origin = `http://127.0.0.1:${server.address().port}`;
    async function pageFor(scenario) {
      const context = await browser.newContext();
      const page = await context.newPage();
      await page.route('**/*', route => new URL(route.request().url()).origin === origin ? route.continue() : route.abort());
      await page.goto(`${origin}/?scenario=${scenario}`);
      await page.getByRole('button', { name: 'MX Bikes', exact: true }).click();
      return { page, context };
    }
    for (const scenario of ['auto', 'manual']) {
      const { page, context } = await pageFor(scenario);
      if (scenario === 'manual') {
        await page.getByRole('button', { name: 'setup.chooseGameFolder', exact: true }).click();
        await page.getByRole('button', { name: 'setup.startBrowsing', exact: true }).click();
      }
      await page.getByTestId('dashboard').waitFor();
      const saved = await page.evaluate(() => JSON.parse(localStorage.getItem('setup-config')));
      assert.equal(saved.setupComplete, true);
      assert.ok(saved.modsPath);
      const calls = await page.evaluate(() => window.setupCalls);
      assert.equal(calls.filter(x => x === 'completeSetup').length, 1);
      await page.reload();
      await page.getByTestId('dashboard').waitFor();
      assert.deepEqual(await page.evaluate(() => window.setupCalls), []);
      console.log(`PASS: ${scenario} first-run setup persists completion and survives renderer reload`);
      await context.close();
    }
    const { page, context } = await pageFor('write-error');
    await page.getByRole('button', { name: 'setup.chooseGameFolder', exact: true }).click();
    await page.getByRole('button', { name: 'setup.startBrowsing', exact: true }).click();
    await page.getByText('Error: Simulated completion write failure', { exact: true }).waitFor();
    assert.equal(await page.getByTestId('dashboard').count(), 0);
    assert.equal(await page.evaluate(() => JSON.parse(localStorage.getItem('setup-config')).setupComplete), false);
    console.log('PASS: completion write failure stays in setup with a visible error');
    await context.close();
  } finally {
    if (browser) await browser.close();
    if (server) await new Promise(resolve => server.close(resolve));
    assert.equal(path.dirname(output), tempParent);
    assert.ok(path.basename(output).startsWith('mxb-setup-flow-'));
    await fs.rm(output, { recursive: true, force: true });
  }
})().catch(error => { console.error(error); process.exitCode = 1; });
