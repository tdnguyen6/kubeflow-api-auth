<template>
  <div class="leaf">
    <div class="leaf__title">{{ k }}</div>
    <div class="leaf__switch">
      <toggle-switch @click="updateVal()" :value="val" />
    </div>
  </div>
</template>

<script>
import ToggleSwitch from "../ToggleSwitch.vue";
export default {
  components: { ToggleSwitch },
  props: {
    k: String,
    v: Boolean,
    jsonPath: Array,
  },
  data() {
    return {
      val: this.$props.v,
    };
  },
  methods: {
    updateVal() {
      if (this.$route.params.id === "") {
        this.val = !this.val;
        this.$store.commit("updateTemplateLeaf", {
          jsonPath: this.$props.jsonPath,
          lastPath: this.k,
          value: this.val,
        });
      }
    },
  },
};
</script>

<style lang="scss" scoped>
.leaf {
  display: flex;
  justify-content: space-between;
  align-items: center;
  &__title {
    text-transform: capitalize;
  }

  &__switch {
    min-width: 10%;
    text-align: right;
  }
}
</style>
