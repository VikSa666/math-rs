import { createApp } from "vue";
import App from "./App.vue";
import init from "math-rs";

init().then(() => createApp(App).mount("#app"));
