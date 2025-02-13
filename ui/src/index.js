const { app, BrowserWindow, ipcMain } = require('electron');
const { spawn } = require('child_process');
const path = require('path');

let win;
let rustProcess;

const INDEX_HTML_FP = "../views/index.html"

function create_window(__load_file_path) {
    const win = new BrowserWindow({
        width: 800,
        height: 600,
        autoHideMenuBar: true,
        webPreferences: {
            nodeIntegration: false,
            contextIsolation: true,
            preload: path.join(__dirname, 'preload.js')
        }
    })

    win.loadFile(__load_file_path)

}

app.whenReady().then(() => {
    create_window(INDEX_HTML_FP)

    // Start Rust backend
    // rustProcess = spawn(path.join(__dirname, 'rust-backend'));

    // rustProcess.stdout.on('data', (data) => {
    //     console.log(`Rust: ${data}`);
    // });

    // rustProcess.stderr.on('data', (data) => {
    //     console.error(`Rust Error: ${data}`);
    // });

    // rustProcess.on('close', (code) => {
    //     console.log(`Rust backend exited with code ${code}`);
    // });

    // ipcMain.handle('send-to-rust', async (_event, request) => {
    //     return new Promise((resolve) => {
    //         rustProcess.stdin.write(JSON.stringify(request) + '\n');
    //         rustProcess.stdout.once('data', (data) => {
    //             resolve(JSON.parse(data.toString()));
    //         });
    //     });
    // });
});

app.on('window-all-closed', () => {
    if (rustProcess) rustProcess.kill();
    if (process.platform !== 'darwin') app.quit();
});
