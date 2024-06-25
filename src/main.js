import { createApp } from 'vue'
import App from './App.vue'
import 'vue3-easy-data-table/dist/style.css'
import './assets/style.css'
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

library.add(faUserSecret)
library.add(faBars)
library.add(faEllipsisVertical)
library.add(faEllipsis)
library.add(faRotate)

const app = createApp(App, { port: 8000 })

app.component('font-awesome-icon', FontAwesomeIcon)

app.config.productionTip = false

app.mount('#app')
