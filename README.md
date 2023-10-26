# YewDebugger

![Makefile CI](https://github.com/JADSN1894/yew-debugger/actions/workflows/makefile.yml/badge.svg)
[![Repository license](https://img.shields.io/github/license/JADSN1894/yew-debugger)](https://github.com/JADSN1894/yew-debugger/blob/dev/LICENSE)
[![Crates.io: yew-debugger](https://img.shields.io/crates/v/yew-debugger.svg)](https://crates.io/crates/yew-debugger)
[![Crates.io: yew-debugger-derive](https://img.shields.io/crates/v/yew-debugger-derive.svg)](https://crates.io/crates/yew-debugger-derive) **(derive)**

**yew-debugger** is a devtools extension for debugging Yew applications that includes the following features:

- It is built upon the minimally invasive implementation of only three traits: `YewComponentModel`, `YewComponentMessage`, and `YewComponentDebug`.
- It offers a detailed view of messages and changes in the Model during state transitions.
- It provides a real-time, straight-forward view of the application's MVU (Model, View, Update) cycles.

## Screenshot
![Screenshot](docs/screenshots/yew-debugger-running-with-example.png?raw=true)

## Getting Started

### <a id="cloning-the-repository"></a> Cloning the repository
1. Clone the repository: `git clone --depth=1 git@github.com:JADSN1894/yew-debugger.git`
1. Change to directory `yew-debugger`: `cd yew-debugger`
1. Open *yew-debugger folder in VsCode*: `code .`
1. Install the extension [**Dev Containers**](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)
1. Open the *Dev Container*
   
![Screenshot](docs/getting-started/01-open-dev-container-in-vscode.png?raw=true)

6. Install the extensions *(Optional)*

![Screenshot](docs/getting-started/02-install-vscode-extensions-at-devcontainer.png?raw=true)

7. When see the *"...Done. Press any ke to close the terminal."* press `c`:

![Screenshot](docs/getting-started/03-after-finish-devcontainer.png?raw=true)

8. Open new terminal:

![Screenshot](docs/getting-started/04-open-new-terminal.png?raw=true)

9. Execute: `make demo`
1. Execute: `python3 -m http.server --directory ./dist-example/`
1. In a chromium-based open extensions window: `chrome://extensions/`
1. At right top corner enable **Developer mode**

![Screenshot](docs/getting-started/05-enable-developer-mode.png?raw=true)


13. Click at `Load unpacked` button
1. Open the `crx` folder, (If comming *from actions artifacts* section, the folder name is `yew-debugger-crx`)
1. Click at `service worker(inactive)`
1. Check if show at least this two logs:
    1. *[From background module]: Hello from YewDebugger*
    1. *Yew Debugger devtools tab clicked*
1. Open in new tab: `http://localhost:8000/`
1. Open DevTools pressing `F12` or `crtl + shift + i`
1. At DevTools panel tab open the tab `Yew Debugger`
1.  And controll your application. 😎

### From actions artifacts

1. Download the artifacts:
   1. `yew-debugger-crx.zip`
   1. `dist-example-yew-debugger-counter-layout.zip`
   1. `dist-example-spreadsheet-cells-with-yew-debugger.zip`
1. Create a folder named: *yew-debugger*  
1. Change to directory: `cd yew-debugger`
1. Extract the compressed files :
   1. `unzip yew-debugger-crx.zip -d crx`
   1. `unzip dist-example-yew-debugger-counter-layout.zip -d dist-example-yew-debugger-counter-layout`
   1. `unzip dist-example-spreadsheet-cells-with-yew-debugger.zip -d dist-example-spreadsheet-cells-with-yew-debugger`
   1. Directory after extract 
   ```
    .
    ├── dist-example-spreadsheet-cells-with-yew-debugger
    │   ├── frontend-wasm
    │   │   ├── index_bg.wasm
    │   │   ├── index.js
    │   │   └── package.json
    │   ├── index.html
    │   ├── index.js
    │   └── output.css
    ├── dist-example-spreadsheet-cells-with-yew-debugger.zip
    ├── dist-example-yew-debugger-counter-layout
    │   ├── assets
    │   │   ├── index-0c5742a5.js
    │   │   ├── index_bg-2f50b49e.wasm
    │   │   └── index-d9ca7bb5.css
    │   ├── favicon.ico
    │   ├── index.html
    │   └── vite.svg
    ├── dist-example-yew-debugger-counter-layout.zip
    ├── yew-debugger-crx
    │   ├── assets
    │   │   ├── index-643717a7.css
    │   │   ├── index-df5db1f8.js
    │   │   └── panel_bg-88b3fac6.wasm
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
    └── yew-debugger-crx.zip
     ```
    3. Follow the steps from **8** of [Cloning the repository](#cloning-the-repository) instructions

## 👍 Acknowledgements

Huge thanks to [afsec](https://github.com/afsec) for giving me the idea for this project, sharing the inspiration behind it, and implementing the first iteration in Rust. Your contributions have been invaluable.

## 📪 Contact

Feel free to reach out to me at [E-mail](mailto:9gdcij581@mozmail.com) to discuss how `yew-debugger` can elevate the quality of your Yew development and improve the end-user experience!
