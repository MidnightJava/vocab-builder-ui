import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import 'vue3-easy-data-table/dist/style.css';
// import "./assets/main.css";

const app = createApp(App);

app.use(router);

app.mount("#app");
