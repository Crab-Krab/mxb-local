import { sanitizeDescriptionHtml } from '../src/lib/descriptionHtml.mjs';

const cases = [
  ['Browse SVG handler', '<svg onload="parent.__auditMarker=1"></svg>'],
  ['Shop form-feed iframe', '<iframe\fsrcdoc="&lt;script&gt;parent.__auditMarker=1&lt;/script&gt;"></iframe>'],
  ['Image handler', '<img src="https://example.invalid/a.png" onerror="parent.__auditMarker=1">'],
  ['JavaScript link', '<a href="java&#10;script:parent.__auditMarker=1">link</a>'],
  ['Encoded frame document', '<iframe srcdoc="&lt;img src=x onerror=parent.__auditMarker=1&gt;"></iframe>'],
  ['Foreign namespace', '<math><mtext><img src=x onerror="parent.__auditMarker=1"></mtext></math>'],
  ['Template', '<template><img src=x onerror="parent.__auditMarker=1"></template>'],
  ['CSS and DOM clobbering', '<a id="__TAURI_INTERNALS__" name="currentScript" style="position:fixed" href="https://example.com/">link</a>'],
];
const results = [];
const allowed = new Set('p br strong b em i u s ul ol li blockquote pre code h1 h2 h3 h4 h5 h6 table thead tbody tfoot tr td th hr a img span div'.split(' '));
for (const [name, raw] of cases) {
  const clean = sanitizeDescriptionHtml(raw);
  const tree = new DOMParser().parseFromString(clean, 'text/html');
  let safe = true;
  for (const element of tree.body.querySelectorAll('*')) {
    if (!allowed.has(element.localName)) safe = false;
    for (const { name: attribute, value } of element.attributes) {
      const ok = (element.localName === 'a' && attribute === 'href' && value.startsWith('https://')) ||
        (element.localName === 'img' && ['src', 'referrerpolicy', 'loading'].includes(attribute));
      if (!ok || attribute.startsWith('on')) safe = false;
    }
  }
  window.__auditMarker = 0;
  // Network access is blocked by the test runner. These markers do not access native APIs.
  document.querySelector('#rendered').innerHTML = clean;
  await new Promise(resolve => setTimeout(resolve, 30));
  results.push({ name, passed: safe && window.__auditMarker === 0, output: clean });
}
const formatted = sanitizeDescriptionHtml('<p>Hello <strong>rider</strong></p><a href="https://example.com/">details</a>');
results.push({ name: 'Formatting and HTTPS links survive', passed: formatted.includes('<strong>rider</strong>') && formatted.includes('href="https://example.com/"') });
const twice = sanitizeDescriptionHtml(formatted);
results.push({ name: 'Sanitization is stable', passed: twice === formatted });
results.push({ name: 'Oversized descriptions are refused', passed: sanitizeDescriptionHtml('x'.repeat(512 * 1024 + 1)) === '' });
const deep = '<div>'.repeat(200) + 'tail' + '</div>'.repeat(200);
results.push({ name: 'Deep descriptions have a bounded output', passed: !sanitizeDescriptionHtml(deep).includes('tail') });
const passed = results.every(r => r.passed);
document.querySelector('#rendered').replaceChildren();
document.querySelector('#result').textContent = JSON.stringify({ passed, tests: results.length, results }, null, 2);
document.documentElement.dataset.testResult = passed ? 'passed' : 'failed';
