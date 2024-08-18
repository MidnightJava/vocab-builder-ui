import { createApp } from 'vue'
import App from './App.vue'
import 'vue3-easy-data-table/dist/style.css'
// import './assets/style.css'
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

const start = async () => {
  //Avoid trying to start the server again on page refresh
  try {
    await useFetch(`http://${host}:5000/alive`, 'GET')
    setupApp()
  } catch (e) {
    startServer()
    setTimeout(() => setupApp(), 500)
  }
}

async function startServer() {
  const Command = window.__TAURI__.shell.Command
  const command = Command.sidecar('binaries/server')
  try {
    const res = await command.execute()
    console.log(`Command result: ${JSON.stringify(res)}`)
  } catch (e) {
    console.log(`Error starting server: ${e.message}`)
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
