import {defineConfig} from "vite";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

export default defineConfig({
    plugins: [
        // this will enable importing of WASM files
        wasm(),

        // this will take care of the async loading of the WASM files
        topLevelAwait()
    ]
})