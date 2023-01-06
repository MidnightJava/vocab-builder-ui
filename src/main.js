import { createApp } from "vue";
import App from "./App.vue";
import 'vue3-easy-data-table/dist/style.css';
// import "./assets/main.css";

/* import the fontawesome core */
import { library } from '@fortawesome/fontawesome-svg-core'

/* import font awesome icon component */
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'

/* import specific icons */
import { faUserSecret } from '@fortawesome/free-solid-svg-icons'
import { faBars } from '@fortawesome/free-solid-svg-icons'

/* add icons to the library */
library.add(faUserSecret)
library.add(faBars);

const app = createApp(App);

/* add font awesome icon component */
app.component('font-awesome-icon', FontAwesomeIcon)

app.config.productionTip = false

app.mount("#app");
