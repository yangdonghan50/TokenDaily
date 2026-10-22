import { readFileSync, writeFileSync, mkdirSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { resolve } from 'node:path';
import { spawnSync } from 'node:child_process';

const root = fileURLToPath(new URL('../', import.meta.url));
const source = readFileSync(resolve(root, 'assets/TokenDaily_logo_gradient.svg'), 'utf8');
// Exact bounds of the outer circular arc: centre (183.091125, 183.977194),
// radius 147.753668. Keep 0.5 SVG units outside every extremum so that the
// entire mark survives rasterization; the drawing fills 99.66% of the canvas.
const traySvg = source.replace('viewBox="0 0 369 369"',
  'viewBox="34.837457 35.723526 296.507336 296.507336"');
if (traySvg === source) throw new Error('Brand SVG viewBox changed; review tray bounds.');
mkdirSync(resolve(root, '.bundle-tmp'), { recursive: true });
const input = resolve(root, '.bundle-tmp/tray-icon.svg');
writeFileSync(input, traySvg);
const result = spawnSync(process.execPath, [resolve(root, 'node_modules/@tauri-apps/cli/tauri.js'),
  'icon', input, '--output', resolve(root, 'src-tauri/icons/tray'), '--png', '64'],
  { cwd: root, stdio: 'inherit' });
if (result.error) throw result.error;
process.exit(result.status ?? 1);
