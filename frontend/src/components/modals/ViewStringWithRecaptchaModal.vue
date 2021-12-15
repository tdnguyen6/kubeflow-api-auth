<template>
  <div class="container">
    <div class="container__str">{{ content.str }}</div>
    <div class="container__copy" @click="copy(content.str)">
      <div>
        <fa v-if="copied" icon="clipboard-check" />
        <fa v-else icon="clipboard" />
      </div>
    </div>
  </div>
</template>

<script>
export default {
  props: {
    content: {},
  },
  data() {
    return {
      copied: false,
    };
  },
  methods: {
    copy(str) {
      navigator.clipboard.writeText(str);
      this.$data.copied = true;
    },
  },
};
</script>

<style lang="scss" scoped>
.container {
  margin: 0 2rem;

  @media (min-width: 900px) {
    margin: 0 4rem;
  }

  @media (min-width: 1500px) {
    margin: 0 6rem;
  }
  border: 1px dashed black;
  display: flex;
  flex-wrap: nowrap;
  &:hover {
    border: 1px solid black;
  }

  &__str {
    padding: 1.5rem;
    white-space: normal;
    background: $gray-1;
    overflow-wrap: break-word;
    word-wrap: break-word;
    word-break: break-all;
    word-break: break-word;
    hyphens: auto;
  }

  &__copy {
    padding: 1rem;
    font-size: 1.5rem;
    border-left: 1px dashed black;
    cursor: pointer;
    display: flex;
    align-items: center;
    background: $gray-3;
    color: $gray-1;

    &:hover {
      background: #000;
    }
  }
}
</style>
