// Remote descriptions are serialized from an inert parsed tree with an allowlist.
// Never preserve attributes or element namespaces supplied by a content service.
const ALLOWED = new Set('p br strong b em i u s ul ol li blockquote pre code h1 h2 h3 h4 h5 h6 table thead tbody tfoot tr td th hr a img span div'.split(' '));
const DROP = new Set('script style iframe frame frameset object embed svg math template noscript form input button textarea select option meta base link title xmp plaintext'.split(' '));
const escapeText = (s) => s.replaceAll('&', '&amp;').replaceAll('<', '&lt;').replaceAll('>', '&gt;').replaceAll('"', '&quot;');
function httpsUrl(value) {
  try { const url = new URL(value); return url.protocol === 'https:' && !url.username && !url.password ? url.href : null; }
  catch { return null; }
}
export function sanitizeDescriptionHtml(raw) {
  if (raw.length > 512 * 1024) return '';
  const doc = new DOMParser().parseFromString(raw, 'text/html');
  function clean(node, depth = 0) {
    if (depth > 64) return '';
    if (node.nodeType === Node.TEXT_NODE) return escapeText(node.textContent || '');
    if (node.nodeType !== Node.ELEMENT_NODE) return '';
    const tag = node.localName;
    if (node.namespaceURI !== 'http://www.w3.org/1999/xhtml' || DROP.has(tag)) return '';
    const children = [...node.childNodes].map(child => clean(child, depth + 1)).join('');
    if (!ALLOWED.has(tag)) return children;
    let attrs = '';
    if (tag === 'a' || tag === 'img') {
      const name = tag === 'a' ? 'href' : 'src';
      const url = httpsUrl(node.getAttribute(name) || '');
      if (url) attrs = ` ${name}="${escapeText(url)}"`;
      if (tag === 'img' && !url) return '';
    }
    if (tag === 'img') attrs += ' referrerpolicy="no-referrer" loading="lazy"';
    if (['img', 'br', 'hr'].includes(tag)) return `<${tag}${attrs}>`;
    return `<${tag}${attrs}>${children}</${tag}>`;
  }
  return [...doc.body.childNodes].map(node => clean(node)).join('');
}
