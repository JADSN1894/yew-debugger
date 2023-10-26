// import './output.css';
import init, { run_app } from './frontend-wasm/index.js';

await init().then(() => run_app());