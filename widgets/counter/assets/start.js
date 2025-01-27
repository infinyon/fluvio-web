import counter_init, * as bindings from '/fluvio-counter/widget-counter-bin.js';
const wasm = await counter_init({ module_or_path: '/fluivio-counter/widget-counter-bin_bg.wasm' });

window.wasmBindings = bindings;
dispatchEvent(new CustomEvent("TrunkApplicationStarted", { detail: { wasm } }));
