<template>
  <transition name="fade">
    <div class="modal" v-if="config.show">
      <div class="modal__main">
        <div class="modal__overlay" @click.self="this.$emit('closeModal')">
          <div class="modal__content">
            <loading-modal v-if="config.type === 'loading'" />
            <view-string-with-recaptcha-modal
              v-else-if="config.type === 'view-string'"
              :content="config.props"
            />
            <success-modal v-else-if="config.type === 'success'" />
            <failure-modal v-else-if="config.type === 'failure'" />
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
import SuccessModal from "./SuccessModal.vue";
import ViewStringWithRecaptchaModal from "./ViewStringWithRecaptchaModal.vue";

export default {
  components: {
    LoadingModal,
    ViewStringWithRecaptchaModal,
    FailureModal,
    SuccessModal,
  },
  data() {
    return {};
  },
  props: {
    config: Object,
  },
  methods: {},
};
</script>

<style lang="scss" scoped>
.modal {
  position: fixed;
  height: 100vh;
  width: 100vw;
  z-index: 99;

  &__main {
  }

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
