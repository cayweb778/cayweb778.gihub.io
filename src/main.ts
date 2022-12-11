import {createApp} from "vue";
import App from "./App.vue";
import router from "./router";
import {store} from "./store";
import {createDesktopGuard} from "./router/guard/desktopGuard";
import {setupConfigKeydownListener} from "./funs";
const app = createApp(App)
app.use(router)
// console.log(998898)
createDesktopGuard(router)
app.use(store)
setupConfigKeydownListener()
app.mount("#app");