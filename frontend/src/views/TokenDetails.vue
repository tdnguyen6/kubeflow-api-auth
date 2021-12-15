<template>
  <div>
    <!-- {{ route.params }} -->
    <modal :config="modalConfig" @closeModal="onCloseModal()" />
    <div class="form">
      <div class="form__title">Token Details</div>
      <div class="form__block block">
        <div class="block__title">Name</div>
        <div class="block__content">
          <div v-if="template" class="input input--text">
            <input
              type="text"
              v-model="name"
              placeholder="Choose a new name for this token"
            />
          </div>
          <div v-else class="text-display">{{ token.name }}</div>
        </div>
      </div>
      <div class="form__block block">
        <div class="block__title block__title--util">
          Rules
          <div
            @click="$refs.ruleNode.shouldExpand(false)"
            class="cta cta--util"
          >
            <fa icon="angle-double-up" />
          </div>
          <div @click="$refs.ruleNode.shouldExpand(true)" class="cta cta--util">
            <fa icon="angle-double-down" />
          </div>
        </div>
        <div class="block__content">
          <token-rule-node ref="ruleNode" :rule="token.rules" :jsonPath="[]" />
        </div>
      </div>
      <div class="form__block block">
        <div class="block__title">Active Time</div>
        <div class="block__content">
          <div class="input input--datepicker" v-if="template">
            <Datepicker
              v-model="date"
              range
              twoCalendars
              :clearable="false"
              format="yyyy-MM-dd"
              >Start Date</Datepicker
            >
          </div>
          <div v-else class="datepicker--show">
            <div class="text-display">
              {{ token.start_date }} <fa icon="arrow-right" />
              {{ token.end_date }}
            </div>
          </div>
        </div>
      </div>
      <div v-if="!template" class="form__block block">
        <div class="block__title">Last Used</div>
        <div class="block__content">
          <div class="text-display">
            {{ token.last_used }}
          </div>
        </div>
      </div>
      <div v-if="template" class="form__block">
        <div class="cta cta--submit">
          <button @click="submit()">Submit</button>
        </div>
      </div>
      <div class="form__back-home">
        <router-link to="/"><fa icon="arrow-left" /> Home</router-link>
      </div>
    </div>
  </div>
</template>

<script>
import Datepicker from "vue3-date-time-picker";
import "vue3-date-time-picker/dist/main.css";
import TokenRuleNode from "@/components/rules/TokenRuleNode.vue";
import Modal from "@/components/modals/Modal.vue";
export default {
  components: {
    Datepicker,
    Modal,
    TokenRuleNode,
  },
  data() {
    return {
      template: this.$route.params.id === "",
      token:
        this.$route.params.id === ""
          ? this.$store.getters.tokenTemplate
          : this.$store.getters.tokenById(this.$route.params.id),
      modalConfig: {},
    };
  },
  computed: {
    date: {
      get() {
        return [
          new Date(this.$data.token.start_date),
          new Date(this.$data.token.end_date),
        ];
      },
      set(date) {
        this.$data.token.start_date = this.dateFmt(date[0]);
        this.$data.token.end_date = this.dateFmt(date[1]);
        this.updateTemplate();
      },
    },
    name: {
      get() {
        return this.$data.token.name;
      },
      set(name) {
        this.$data.token.name = name;
        this.updateTemplate();
      },
    },
  },
  methods: {
    updateTemplate() {
      this.$store.commit("updateTemplate", this.$data.token);
    },
    onCloseModal() {},
    onCloseModalDefault() {
      this.modalConfig.show = false;
    },
    async submit() {
      if (this.$store.getters.tokenTemplate.name === "") {
        alert("Please give this new token a name");
        return;
      }

      this.$data.modalConfig = {
        show: true,
        type: "loading",
      };
      try {
        await fetch(
          `${process.env.VUE_APP_BACKEND_HOST}/api/token?email=${this.$store.getters.userId}&recaptchaToken=${this.$store.getters.recaptchaToken}`,
          {
            method: "POST",
            headers: {
              "Content-Type": "application/json",
            },
            body: JSON.stringify(this.$store.getters.tokenTemplate),
          }
        );
        this.$data.modalConfig = {
          show: true,
          type: "success",
        };

        this.onCloseModal = () => {
          sessionStorage.rerender = true;
          this.$router.push({ path: "/" });
        };
      } catch (e) {
        console.log(e);
        this.$data.modalConfig = {
          show: true,
          type: "failure",
        };
        this.onCloseModal = this.onCloseModalDefault;
      }
    },
    dateFmt(date) {
      return date.toLocaleDateString("en-CA");
    },
  },
};
</script>

<style lang="scss" scoped>
.form {
  text-align: left;
  width: 90%;
  margin: 3rem auto;

  @media (min-width: 900px) {
    width: 75%;
  }

  @media (min-width: 1500px) {
    width: 50%;
  }

  &__back-home {
    font-size: small;
    padding: 1rem;
  }

  &__title {
    font-size: 2rem;
    margin: 1rem;
    font-weight: bold;
    text-align: center;
  }

  &__block {
    border-top: 1px solid $gray-1;
    padding: 1rem 0;

    .block {
      &__title {
        font-size: 1.25rem;
        font-weight: bold;
        padding: 0.5rem 0;

        &--util {
          display: flex;
          align-items: center;
          gap: 0.5rem;
        }
      }

      &__content {
        .input {
          &--datepicker {
          }

          &--text {
            input {
              width: 100%;
              padding: 0.5rem;
              width: 100%;
              border-radius: 0.25rem;
              border: 1px solid $gray-2;
              font-size: 1rem;
            }
          }
        }

        .text-display {
          border-bottom: 1px solid $gray-2;
          padding: 0.5rem;
          border-radius: 0.25rem;
          background: $gray-1;
        }
      }
    }

    .cta {
      &--submit {
        text-align: center;
        button {
          cursor: pointer;

          color: $gray-1;
          border: none;
          padding: 0.5rem 1rem;
          border-radius: 0.25rem;
          background: $blue-cta;

          &:hover {
            background: $blue-cta-hover;
          }
        }
      }

      &--util {
        cursor: pointer;
        padding: 0.25rem 0.75rem;
        border: 1px solid transparent;
        border-radius: 0.25rem;

        &:hover {
          background: $gray-2;
          border-color: black;
        }
      }
    }
  }
}
</style>
