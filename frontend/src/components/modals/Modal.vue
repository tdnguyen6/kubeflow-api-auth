<template>
  <transition name="fade">
    <div class="modal" v-if="config.show">
      <div class="modal__main">
        <div class="modal__overlay" @click.self="this.$emit('closeModal')">
          <div class="modal__content">
            <loading-modal v-if="config.type === 'loading'" />
            <view-string-modal
              v-else-if="config.type === 'view-string'"
              :content="config.props"
            />
            <success-modal v-else-if="config.type === 'success'" />
            <failure-modal v-else-if="config.type === 'failure'" />
            <recaptcha-v-2-modal
              v-else-if="config.type === 'recaptcha'"
              @recaptchaVerify="$emit('recaptchaVerify')"
            />
            <div v-else>{{ config.type }}</div>
          </div>
        </div>
      </div>
    </div>
  </transition>
</template>

<script>
import FailureModal from "./FailureModal.vue";
import LoadingModal from "./LoadingModal.vue";
import RecaptchaV2Modal from "./RecaptchaV2Modal.vue";
import SuccessModal from "./SuccessModal.vue";
import ViewStringModal from "./ViewStringModal.vue";

export default {
  components: {
    LoadingModal,
    ViewStringModal,
    FailureModal,
    SuccessModal,
    RecaptchaV2Modal,
  },
  data() {
    return {};
  },
  props: {
    config: Object,
  },
  methods: {
    recaptchaFailed(a) {
      console.log(a);
      this.$store.commit("setRecaptchaToken", null);
    },
  },
};
</script>

<style lang="scss" scoped>
.modal {
  position: fixed;
  height: 100vh;
  width: 100vw;
  z-index: 99;

  // &__main {
  // }

  &__overlay {
    width: 100%;
    height: 100%;
    position: fixed;
    top: 0;
    left: 0;
    background: $gray-2;
    opacity: 0.85;
    display: grid;
    justify-content: center;
    align-items: center;
  }

  &__content {
    opacity: 1;
  }
}
</style>
