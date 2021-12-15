<template>
  <div class="home">
    <h2 class="title">API Access Control Settings</h2>
    <modal :config="modal" @closeModal="onCloseModal()" />
    <div class="container">
      <div class="container__top">
        <div class="container__description">
          <div class="description">
            <div class="description__title">API Tokens</div>
            <div class="description__detail">
              Route-based access control following the least privileges
              principle
            </div>
          </div>
        </div>
        <div class="container__cta">
          <div class="container__cta-main">
            <router-link to="/token"
              ><button><fa icon="plus" /> Create Token</button></router-link
            >
          </div>
        </div>
      </div>
      <div class="container__body">
        <div class="api-token-table">
          <table>
            <thead>
              <tr>
                <th>
                  Name
                  <span @click="sortName()" class="cta">
                    <fa v-if="nameSort == 1" icon="sort-up" />
                    <fa v-else-if="nameSort == -1" icon="sort-down" />
                    <fa v-else icon="sort" />
                  </span>
                  <span
                    @click="filterName()"
                    :class="`cta ${nameFilter.active ? 'cta--active' : ''}`"
                  >
                    <fa icon="filter" />
                  </span>
                  <transition name="fade">
                    <div v-if="nameFilter.active" class="cta__filter--text">
                      <input
                        @keyup="updateView()"
                        type="text"
                        v-model="nameFilter.key"
                      />
                    </div>
                  </transition>
                </th>
                <th>Active Date</th>
                <th>Last used</th>
                <th>
                  Status
                  <span
                    @click="filterStatus()"
                    :class="`cta ${statusFilter.active ? 'cta--active' : ''}`"
                    ><fa icon="filter"
                  /></span>
                  <transition name="fade">
                    <div v-if="statusFilter.active" class="cta__filter--select">
                      <select @change="updateView()" v-model="statusFilter.key">
                        <option value="Active">Active</option>
                        <option value="Expired">Expired</option>
                        <option value="All">All</option>
                      </select>
                    </div>
                  </transition>
                </th>
                <th></th>
              </tr>
            </thead>
            <tbody>
              <transition-group name="fade">
                <tr v-for="(t, i) in tokensView" :key="i">
                  <td>{{ t.name }}</td>
                  <td>
                    {{ t.start_date }} <fa icon="arrow-right" />
                    {{ t.end_date }}
                  </td>
                  <td>{{ t.last_used }}</td>
                  <td>
                    <div
                      v-if="expired(t.end_date)"
                      class="status-badge status-badge--active"
                    >
                      Active
                    </div>
                    <div v-else class="status-badge status-badge--expired">
                      Expired
                    </div>
                  </td>
                  <td>
                    <div class="api-token-table__cta-container">
                      <button
                        @click="popUp[i] = !popUp[i]"
                        @blur="popUp[i] = false"
                        class="api-token-table__cta"
                      >
                        <fa icon="ellipsis-h" />
                      </button>
                      <!-- <div > -->
                      <transition name="fade">
                        <div v-if="popUp[i]" class="pop-up">
                          <div class="pop-up__row" @click="viewToken(t.id)">
                            <fa icon="eye" /> View
                          </div>
                          <router-link :to="`/token/${i}`">
                            <div class="pop-up__row">
                              <fa icon="info-circle" /> Details
                            </div>
                          </router-link>
                          <div class="pop-up__row" @click="deleteToken(t.id)">
                            <fa icon="trash" /> Delete
                          </div>
                        </div>
                      </transition>
                    </div>
                  </td>
                </tr>
              </transition-group>
            </tbody>
          </table>
        </div>
      </div>
    </div>
    <div class="container">
      <div class="container__top no-border-bottom">
        <div class="container__description">
          <div class="description">
            <div class="description__title">API Key</div>
            <div class="description__detail">
              Unlimited access to all API endpoints
            </div>
          </div>
        </div>
        <div class="container__cta">
          <div class="container__cta-main">
            <button @click="viewKey()"><fa icon="eye" /> View</button>
            <button @click="rollKey()" class="orange-cta">
              <fa icon="dice" /> Roll
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
// @ is an alias to /src
import Modal from "@/components/modals/Modal.vue";

export default {
  name: "Home",
  components: {
    Modal,
  },
  data() {
    return {
      modal: {
        show: false,
        type: "",
        props: {
          str: "",
        },
      },
      popUp: new Array(this.$store.getters.tokenListLength).fill(false),
      tokens: this.$store.getters.tokenList,
      tokensView: this.$store.getters.tokenList,
      shouldRerender: false,
      nameSort: 0,
      statusFilter: {
        active: false,
        key: "All",
      },
      nameFilter: {
        active: false,
        key: "",
      },
    };
  },
  methods: {
    async viewKey() {
      let res = await fetch(
        `${process.env.VUE_APP_BACKEND_HOST}/api/key?email=${this.$store.getters.userId}&recaptchaToken=${this.$store.getters.recaptchaToken}`,
        {
          method: "POST",
        }
      );
      let str = await res.text();
      this.$data.modal = {
        show: true,
        type: "view-string",
        props: {
          str: str,
        },
      };
    },
    async rollKey() {
      let res = await fetch(
        `${process.env.VUE_APP_BACKEND_HOST}/api/key/roll?email=${this.$store.getters.userId}&recaptchaToken=${this.$store.getters.recaptchaToken}`,
        {
          method: "POST",
        }
      );
      let str = await res.text();
      this.$data.modal = {
        show: true,
        type: "view-string",
        props: {
          str: str,
        },
      };
    },
    async viewToken(id) {
      let res = await fetch(
        `${process.env.VUE_APP_BACKEND_HOST}/api/token/${id}?recaptchaToken=${this.$store.getters.recaptchaToken}`,
        {
          method: "POST",
        }
      );
      let str = await res.text();
      this.$data.modal = {
        show: true,
        type: "view-string",
        props: {
          str: str,
        },
      };
    },
    async deleteToken(id) {
      try {
        let res = await fetch(
          `${process.env.VUE_APP_BACKEND_HOST}/api/token/${id}?recaptchaToken=${this.$store.getters.recaptchaToken}`,
          {
            method: "DELETE",
          }
        );
        console.log(res);
        if (res.ok) {
          this.$data.modal = {
            show: true,
            type: "success",
          };
          this.$data.shouldRerender = true;
        } else {
          throw "request not ok";
        }
      } catch (e) {
        this.$data.modal = {
          show: true,
          type: "failure",
        };
      }
    },
    expired(date) {
      return new Date().toLocaleDateString("en-CA") < date;
    },
    onCloseModal() {
      this.$data.modal.show = false;
      if (this.$data.shouldRerender) {
        this.$data.shouldRerender = false;
        this.$router.go();
      }
    },
    sortName() {
      let s = this.$data.nameSort;
      if (s == 1) s = -1;
      else if (s == -1) s = 0;
      else s = 1;
      this.$data.nameSort = s;
      this.updateView();
    },
    filterStatus() {
      this.$data.statusFilter.active = !this.$data.statusFilter.active;
    },
    filterName() {
      this.$data.nameFilter.active = !this.$data.nameFilter.active;
    },
    matchStatus(token) {
      if (this.$data.statusFilter.key != "All") {
        let expired = this.expired(token.end_date);
        if (this.$data.statusFilter.key == "Active") {
          return expired;
        }
        return !expired;
      }
      return true;
    },
    updateView() {
      // filter
      let filteredData = this.$data.tokens.filter(
        (t) => t.name.includes(this.$data.nameFilter.key) && this.matchStatus(t)
      );

      const s = this.$data.nameSort;
      // sort
      if (s == 1) {
        filteredData = filteredData.sort((a, b) => (a.name > b.name ? 1 : -1));
      } else if (s == -1) {
        filteredData = filteredData.sort((a, b) => (a.name > b.name ? -1 : 1));
      }
      this.$data.tokensView = filteredData;
    },
  },
  beforeCreate() {
    if (sessionStorage.rerender) {
      sessionStorage.removeItem("rerender");
      this.$router.go();
    }
  },
};
</script>

<style lang="scss" scoped>
@mixin th-td {
  padding: 1rem;
  white-space: nowrap;
}

.home {
  width: 90%;
  margin: 3rem auto;

  @media (min-width: 900px) {
    width: 75%;
  }

  @media (min-width: 1500px) {
    width: 50%;
  }

  .container {
    border: 1px solid $gray-2;
    margin: 2rem auto;
    border-radius: 0.25rem;

    &__top {
      border-bottom: 1px solid $gray-2;
      display: flex;
      justify-content: space-between;
    }

    &__body {
      .api-token-table {
        overflow: scroll;
        max-height: 50vh;
        table {
          border-collapse: collapse;
          width: 100%;
          text-align: left;
          thead {
            background: $gray-1;
            position: sticky;
            top: 0px;
            z-index: 2;
            th {
              @include th-td;
              .cta {
                padding: 0.2rem 0.5rem;
                border-radius: 0.25rem;
                margin-left: 0.25rem;
                cursor: pointer;
                border: 1px solid transparent;

                &:hover,
                &--active {
                  background: $gray-2;
                  border: 1px solid $gray-3;
                }

                &__filter {
                  &--text {
                    width: 6rem;
                  }
                }
              }
              input[type="text"],
              select {
                width: 100%;
                padding: 0.5rem;
                margin-top: 1rem;
                border-radius: 0.25rem;
                border: 1px solid $gray-2;
              }
            }
          }

          tbody {
            tr {
              &:hover {
                background: $yellow-highlight-row;
              }

              td {
                @include th-td;
              }
            }
          }
        }

        .status-badge {
          padding: 0.25rem 0.5rem;
          font-size: 0.75rem;
          font-weight: bold;
          border-radius: 1rem;
          width: fit-content;

          &--active {
            background: $green-badge;
          }

          &--expired {
            background: $red-badge;
          }
        }

        &__cta-container {
          position: relative;

          .pop-up {
            position: absolute;
            top: 40px;
            left: -4rem;
            border: 1px solid $gray-2;
            padding: 0.5rem 0rem;
            border-radius: 0.25rem;
            z-index: 2;
            background: white;
            a {
              text-decoration: none;
            }

            &::after {
              content: "";
              width: 0;
              height: 0;
              border-width: 10px;
              border-style: solid;
              border-color: transparent transparent #ffffff transparent;
              position: absolute;
              top: -20px;
              left: 69px;
              z-index: 1;
            }

            &::before {
              content: "";
              width: 0;
              height: 0;
              border-width: 11px;
              border-style: solid;
              border-color: transparent transparent $gray-2 transparent;
              position: absolute;
              top: -22px;
              left: 68px;
            }

            &__row {
              padding: 0.5rem 1rem;
              color: $blue-cta;
              cursor: pointer;
              &:hover {
                background: $gray-2;
              }
            }
          }
        }
        &__cta {
          border: none;
          cursor: pointer;
          padding: 0.5rem 1rem;
          border-radius: 0.25rem;
          font-size: 0.75rem;

          &:hover {
            background: $gray-2;
          }
        }
      }
    }

    &__description {
      display: flex;
      align-items: center;
    }

    &__cta {
      border-left: 1px solid $gray-2;
      background: $gray-1;
      display: flex;
      justify-content: center;
      align-items: center;
      width: 40%;
    }

    &__cta-main {
      width: 100%;
      display: flex;
      flex-wrap: wrap;
      gap: 2rem;
      margin: 2rem;
      justify-content: center;
      button {
        background: $blue-cta;
        color: $gray-1;
        padding: 0.5rem 0.75rem;
        border-radius: 0.25rem;
        border: none;
        &:hover {
          background: $blue-cta-hover;
        }
        cursor: pointer;
      }
      .orange-cta {
        background: $orange-cta;
        &:hover {
          background: $orange-cta-hover;
        }
      }
    }
  }

  .description {
    text-align: left;
    margin: 1.5rem;

    &__title {
      font-size: 1.2rem;
      font-weight: bold;
      margin-bottom: 1rem;
    }

    &__detail {
      font-size: 0.9rem;
      line-height: 1.5rem;
    }
  }

  .title {
    margin: 2rem auto;
  }

  .no-border-bottom {
    border-bottom: none;
  }
}
</style>
