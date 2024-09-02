import { createApp, ref } from 'vue'
import App from './App.vue'
import 'vue3-easy-data-table/dist/style.css'
import './assets/main.css'

/* import the fontawesome core */
import { library } from '@fortawesome/fontawesome-svg-core'

/* import font awesome icon component */
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'

import { faUserSecret } from '@fortawesome/free-solid-svg-icons'
import { faBars } from '@fortawesome/free-solid-svg-icons'
import { faEllipsisVertical } from '@fortawesome/free-solid-svg-icons'
import { faEllipsis } from '@fortawesome/free-solid-svg-icons'
import { faRotate } from '@fortawesome/free-solid-svg-icons'
import { useFetch } from './components/fetch.js'

const host = 'localhost'
const DEFAULT_PORT = 5023
const env = import.meta.env
const portVal = env?.VITE_SERVER_PORT || DEFAULT_PORT
const port = ref(portVal)

const start = async () => {
  // Give the backend time to start up if needed
  try {
    await useFetch(`http://${host}:${port.value}/alive`, 'GET')
    setupApp()
  } catch (e) {
    setTimeout(() => setupApp(), 1000)
  }
}

const setupApp = () => {
  const app = createApp(App)
  app.component('font-awesome-icon', FontAwesomeIcon)
  app.config.productionTip = false
  app.mount('#app')
}

library.add(faUserSecret)
library.add(faBars)
library.add(faEllipsisVertical)
library.add(faEllipsis)
library.add(faRotate)

start()
