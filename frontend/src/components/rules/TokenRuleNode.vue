<template>
  <div class="rule">
    <div v-for="(v, k) in rule" :key="k" class="rule__block block">
      <div v-if="typeof v === 'boolean'" class="block__leaf">
        <token-rule-leaf :k="k" :v="v" :jsonPath="jsonPath" />
      </div>
      <div v-else class="block__node node">
        <div @click.self="expand[k] = !expand[k]" class="node__title">
          {{ k }}
        </div>
        <transition name="fade">
          <div v-show="expand[k]" class="node__content">
            <token-rule-node
              :ref="setRuleNode"
              :rule="v"
              :jsonPath="jsonPath.concat([k])"
            />
          </div>
        </transition>
      </div>
    </div>
  </div>
</template>

<script>
import TokenRuleNode from "@/components/rules/TokenRuleNode.vue";
import TokenRuleLeaf from "@/components/rules/TokenRuleLeaf.vue";

export default {
  components: { TokenRuleNode, TokenRuleLeaf },
  data() {
    return {
      expand: Object.fromEntries(Object.keys(this.rule).map((k) => [k, true])),
      ruleNodes: [],
    };
  },
  props: {
    rule: {},
    jsonPath: Array,
  },
  methods: {
    shouldExpand(e) {
      for (const k in this.expand) {
        this.expand[k] = e;
      }
      this.ruleNodes.forEach((n) => n.shouldExpand(e));
    },
    setRuleNode(n) {
      if (n) {
        this.ruleNodes.push(n);
      }
    },
  },
};
</script>

<style lang="scss" scoped>
.rule {
  &__block {
    .block {
      &__node {
        .node {
          &__title {
            font-weight: bold;
            background: $gray-1;
            padding: 1rem;
            margin: 0.5rem 0;
            border-radius: 0.25rem;
            cursor: pointer;
            border: 1px solid $gray-3;
            text-transform: uppercase;

            &:hover {
              background: $gray-2;
            }
          }

          &__content {
            padding-left: 1.5rem;
            margin-left: 1.5rem;
            border-left: 1px solid black;
          }
        }
      }

      &__leaf {
        padding: 1rem;

        &:hover {
          background: $yellow-highlight-row;
        }
      }
    }
  }
}
</style>
