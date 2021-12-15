import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import store from "./store";
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";
import { library } from "@fortawesome/fontawesome-svg-core";
import {
  faEye,
  faPlus,
  faDice,
  faArrowRight,
  faSortUp,
  faSortDown,
  faSort,
  faFilter,
  faEllipsisH,
  faTrash,
  faInfoCircle,
  faClipboard,
  faClipboardCheck,
  faCheck,
  faExclamation,
  faAngleDoubleDown,
  faAngleDoubleUp,
  faArrowLeft,
} from "@fortawesome/free-solid-svg-icons";

library.add(faPlus);
library.add(faArrowRight);
library.add(faDice);
library.add(faEye);
library.add(faSortUp);
library.add(faSortDown);
library.add(faSort);
library.add(faFilter);
library.add(faEllipsisH);
library.add(faTrash);
library.add(faInfoCircle);
library.add(faClipboardCheck);
library.add(faClipboard);
library.add(faCheck);
library.add(faExclamation);
library.add(faAngleDoubleDown);
library.add(faAngleDoubleUp);
library.add(faArrowLeft);




createApp(App)
  .use(store)
  .use(router)
  .component("fa", FontAwesomeIcon)
  .mount("#app");
