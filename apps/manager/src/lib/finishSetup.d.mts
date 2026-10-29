export function finishSetup(actions: {
  saveFolders: () => Promise<unknown>;
  completeSetup: () => Promise<unknown>;
  onComplete: () => void | Promise<void>;
  onIntegration?: () => void;
}): Promise<void>;
