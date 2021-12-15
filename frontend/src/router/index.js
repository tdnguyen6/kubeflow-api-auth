import { createRouter, createWebHashHistory } from "vue-router";
import Home from "../views/Home.vue";
import TokenDetails from "../views/TokenDetails.vue";
import PageNotFound from "../views/PageNotFound.vue";

const routes = [
  {
    path: "/",
    name: "Home",
    component: Home,
  },
  {
    path: "/token/:id?",
    name: "Token Details",
    // route level code-splitting
    // this generates a separate chunk (about.[hash].js) for this route
    // which is lazy-loaded when the route is visited.
    component: TokenDetails,
  },
  { path: "/:catchAll(.*)", component: PageNotFound },
];

const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

export default router;
