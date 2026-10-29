// Browser integration fixture: only native APIs are mocked. Setup.tsx and its
// real React effects/callbacks run unchanged. No real game files are touched.
const scenario = new URLSearchParams(location.search).get('scenario') ?? 'auto';
const modsPath = 'C:\\Test\\PiBoSo\\MX Bikes';
const calls: string[] = [];
Object.assign(window, { setupCalls: calls });
export const readConfig = () => JSON.parse(localStorage.getItem('setup-config') || 'null');
const writeConfig = (value: unknown) => localStorage.setItem('setup-config', JSON.stringify(value));
export async function createConfig(config: { modsPath: string; gamePath: string; activeGame: string }) {
  calls.push('createConfig');
  if (!config.modsPath && scenario !== 'auto') throw new Error('No detected folder');
  writeConfig({ ...config, modsPath: config.modsPath || modsPath, setupComplete: false });
  return true;
}
export async function completeSetup() {
  calls.push('completeSetup');
  if (scenario === 'write-error') throw new Error('Simulated completion write failure');
  const config = readConfig();
  if (!config?.modsPath) throw new Error('No saved folder');
  writeConfig({ ...config, setupComplete: true });
}
export const detectGamePath = async () => 'C:\\Test\\Steam\\MX Bikes';
export const inspectGameFolder = async (path: string) => ({
  path: path || modsPath, expected: modsPath, exists: true, usable: true,
  matchesExpected: true, correction: null,
});
export const normalizeGameFolder = async (path: string) => ({ path, correction: null });
export const checkInstallFolder = async (path: string) => path;
export const setGamePath = async (path: string) => path;
export const open = async () => modsPath;
export const usePlatform = () => 'windows';
export const useFrostmod = () => ({
  enableIntegration: async () => { throw new Error('Native integration must not run during app-only setup'); },
  useAppOnly: async () => {},
});
export const Mxbmrp3Prompt = () => null;
export const useT = () => (key: string) => key;
export const Trans = ({ k }: { k: string }) => <span>{k}</span>;
