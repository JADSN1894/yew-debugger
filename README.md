# YewDebugger

An Elm-like debugger DevTools extension for Yew.

## Status
![Makefile CI](https://github.com/JADSN1894/yew-debugger/actions/workflows/makefile.yml/badge.svg)

## Screenshot
![Screenshot](docs/screenshots/yew-debugger-running-with-example.png?raw=true)

## Getting Started

### <a id="cloning-the-repository"></a> Cloning the repository
1. Clone the repository: `git clone --depth=1 git@github.com:JADSN1894/yew-debugger.git`
1. Move to directory `yew-debugger`: `cd yew-debugger`
1. Open in VsCode: `code .`
1. Open in DevContainer 
   
![Screenshot](docs/getting-started/01-open-dev-container-in-vscode.png?raw=true)

4. Install the extensions *(Optional)*

![Screenshot](docs/getting-started/02-install-vscode-extensions-at-devcontainer.png?raw=true)

5. When see the *"...Done. Press any ke to close the terminal."* press `c`:

![Screenshot](docs/getting-started/03-after-finish-devcontainer.png?raw=true)

6. Open new terminal:

![Screenshot](docs/getting-started/04-open-new-terminal.png?raw=true)

7. Execute: `make demo`
1. Execute: `python3 -m http.server --directory ./dist-example/`
1. In a chromium-based open extensions window: `chrome://extensions/`
1. Click at `Load unpacked` button
1. Open the `crx` folder
1. Click at `service worker(inactive)`
1. Check if show at least this two logs:
    1. *[From background module]: Hello from YewDebugger*
    1. *Yew Debugger devtools tab clicked*
1. Open in new tab: `http://localhost:8000/`
1. Open DevTools pressing `F12` or `crtl + shift + i`
1. At DevTools panel tab open the tab `Yew Debugger`
1. And controll your application. 😎

### From actions artifacts

1. Download the artifacts:
   1. `yew-app-example.zip`
   1. `yew-debugger.crx`
   
1. Extract the compressed files :
   1. `unzip yew-debugger.crx.zip -d crx`
   2. `unzip yew-app-example.zip.zip -d dist-example`
   3. Directory after extract 
   ```
    ├── crx
    │   ├── assets
    │   │   ├── index-63dd7a97.js
    │   │   ├── index-ed7813bc.css
    │   │   └── panel_bg-7c570c26.wasm
    │   ├── background
    │   │   ├── background_bg.wasm
    │   │   ├── background.js
    │   │   └── package.json
    │   ├── background.js
    │   ├── content-script.js
    │   ├── devtools.html
    │   ├── devtools.js
    │   ├── manifest.json
    │   └── panel.html
    ├── dist-example
    │   ├── assets
    │   │   ├── index-55ebed22.js
    │   │   ├── index_bg-1410e969.wasm
    │   │   └── index-d9ca7bb5.css
    │   ├── favicon.ico
    │   ├── index.html
    │   └── vite.svg
    ├── yew-app-example.zip.zip
    └── yew-debugger.crx.zip
     ```
    4. Follow the steps from **8** of [Cloning the repository](#cloning-the-repository) instructions

## 👍 Acknowledgements

Huge thanks to [afsec](https://github.com/afsec) for giving me the idea for this project, sharing the inspiration behind it, and implementing the first iteration in Rust. Your contributions have been invaluable.

## 📪 Contact

Feel free to reach out to me at [E-mail](mailto:9gdcij581@mozmail.com) to discuss how `yew-debugger` can elevate the quality of your Yew development and improve the end-user experience!
