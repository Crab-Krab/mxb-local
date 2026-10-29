import { useCallback, useState } from 'react';
import { createRoot } from 'react-dom/client';
import Setup from '../src/Components/Setup/Setup';
import { readConfig } from './setup-flow-mocks';

const games = [
  { id: 'mxb', display: 'MX Bikes' },
  { id: 'gpb', display: 'GP Bikes' },
] as any;
function Fixture() {
  const [config, setConfig] = useState(readConfig);
  const reload = useCallback(() => setConfig(readConfig()), []);
  if (config?.modsPath && config.setupComplete) {
    return <main data-testid="dashboard">Dashboard ready</main>;
  }
  return <Setup game={games[0]} games={games} firstRun={!config?.setupComplete} onComplete={reload} />;
}
createRoot(document.getElementById('root')!).render(<Fixture />);
