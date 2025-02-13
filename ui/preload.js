const { contextBridge, ipcRenderer } = require('electron');

contextBridge.exposeInMainWorld('api', {
    sendToRust: (action) => ipcRenderer.invoke('send-to-rust', { action })
});
