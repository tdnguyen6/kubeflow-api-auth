<template>
  <router-view v-if="ready" />
  <modal v-else :config="modalConfig" />
</template>

<script>
import Modal from "./components/modals/Modal.vue";

export default {
  components: {
    Modal,
  },
  data() {
    return {
      ready: false,
      modalConfig: {
        show: true,
        type: "loading",
      },
    };
  },
  async created() {
    this.$store.commit("setUserId", sessionStorage.userid);
    sessionStorage.removeItem("userid");

    let res = await fetch(
      `${process.env.VUE_APP_BACKEND_HOST}/api/token/list?email=${this.$store.state.userid}`
    );

    this.$store.commit("initTokenData", await res.json());
    this.$data.ready = true;
  },
};
</script>

<style lang="scss">
@import url("https://fonts.googleapis.com/css2?family=Roboto:wght@300&display=swap");

body {
  margin: 0;
  padding: 0;
  * {
    box-sizing: border-box;
  }
  background: white;
}

#app {
  font-family: Roboto, Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.5s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
