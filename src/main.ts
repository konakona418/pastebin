import { createApp } from "vue";
import App from "./App.vue";

import mitt from "mitt";

export let app = createApp(App)
app.config.globalProperties.$eventBus = mitt();
app.mount("#app");