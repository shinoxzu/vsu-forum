import { createApp } from "vue";
import { createPinia } from 'pinia'
import PrimeVue from "primevue/config";
import Aura from "@primevue/themes/aura";
import App from "./App.vue";
import router from "./router";
import "./assets/main.css";
import "primeicons/primeicons.css";

async function loadLocales() {
    const primeReactLocaleFiles = import.meta.glob(
        "../node_modules/primelocale/*.json",
    );

    const localeFiles = {};

    for (const path in primeReactLocaleFiles) {
        if (Object.prototype.hasOwnProperty.call(primeReactLocaleFiles, path)) {
            const fileName = path.split("/").pop()?.replace(".json", "");
            if (fileName) {
                const fileModule = await primeReactLocaleFiles[path]();
                localeFiles[fileName] = fileModule.default;
            }
        }
    }

    return localeFiles;
}

const localeFiles = await loadLocales();

const app = createApp(App);
app.use(createPinia())
app.use(PrimeVue, {
    theme: {
        preset: Aura,
    },
    locale: localeFiles.ru.ru,
});
app.use(router);
app.mount("#app");
