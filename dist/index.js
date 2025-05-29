// For Tauri v2
const { getCurrentWindow } = window.__TAURI__.window;

document.getElementById('close-btn').addEventListener('click', async () => {
    const appWindow = getCurrentWindow();
    await appWindow.close();
});