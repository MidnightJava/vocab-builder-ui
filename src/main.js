import { createApp } from 'vue'
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

// import { listen } from '@tauri-apps/api/event'
// import { appWindow } from '@tauri-apps/api/window'
// const appWindow = window.__TAURI__.window
// const { exit } = window.__TAURI__.process

const host = 'localhost'

// const CLOSE_REQUESTED = 'tauri://close-requested'

// window.addEventListener('load', () => {
//   listen('tauri://close-requested', async () => {
//     console.log('CLOSE REQUESTED')
//     killServer()

//     // Delay closing to ensure the beacon is sent
//     setTimeout(() => {
//       appWindow.close()
//     }, 1000) // Adjust the timeout as needed
//   })
// })

// window.addEventListener('beforeunload', event => {
//   killServer()
//   // Optional: Prevent the default close behavior to delay the closure
//   event.preventDefault()
//   event.returnValue = '' // Required for Chrome
// })

// exit().then(() => {
//   console.log('App is exiting')
//   killServer()

//   // Allow the app to exit after the beacon is sent
//   setTimeout(() => {
//     appWindow.close()
//   }, 1000) // Adjust as needed
// })

// const killServer = () => {
//   try {
//     navigator.sendBeacon(`http://${host}:5000/kill`)
//   } catch (e) {
//     console.log(`Error when killing server: ${e.message}`)
//   }
// }

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
  // const Command = window.__TAURI__.shell.Command
  // const command = Command.sidecar('binaries/server')
  // try {
  //   const res = await command.execute()
  //   console.log(`Command result: ${JSON.stringify(res)}`)
  // } catch (e) {
  //   console.log(`Error starting server: ${e.message}`)
  // }
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
