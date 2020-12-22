import Vue from "vue";
import App from "./App.vue";
import { provide } from "./provide";
import store from "./store";
import vuetify from "./plugins/vuetify";

Vue.config.productionTip = false;

new Vue({
  provide,
  store,
  vuetify,
  render: h => h(App)
}).$mount("#app");
