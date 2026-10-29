// Save the folders before recording completion. App-only setup has no integration
// screen, so it must perform the final write itself before reloading the dashboard.
export async function finishSetup({ saveFolders, completeSetup, onComplete, onIntegration }) {
  await saveFolders();
  if (onIntegration) {
    onIntegration();
    return;
  }
  await completeSetup();
  await onComplete();
}
