<template>
  <vue-recaptcha
    :sitekey="sitekey"
    @verify="recaptchaVerify"
    @expired="recaptchaExpired"
    @error="recaptchaError"
  ></vue-recaptcha>
</template>

<script>
import { VueRecaptcha } from "vue-recaptcha";
export default {
  components: { VueRecaptcha },
  data() {
    return {
      sitekey: process.env.VUE_APP_RECAPTCHA_SITEKEY,
    };
  },
  methods: {
    recaptchaVerify(res) {
      this.$store.commit("setRecaptchaToken", res);
      this.$emit("recaptchaVerify");
    },
    recaptchaExpired(a) {
      console.log(`Recaptcha expired with arg: ${JSON.stringify(a)}`);
    },
    recaptchaError(a) {
      console.log(`Recaptcha error with arg: ${JSON.stringify(a)}`);
    },
  },
};
</script>
