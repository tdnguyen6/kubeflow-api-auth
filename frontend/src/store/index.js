import { createStore } from "vuex";

export default createStore({
  state: {
    tokenData: {},
    userid: "",
    recaptchaToken: null,
  },
  getters: {
    tokenList(state /*getters*/) {
      return JSON.parse(JSON.stringify(state.tokenData.tokens));
    },
    tokenById: (state /*getters*/) => (id) =>
      JSON.parse(JSON.stringify(state.tokenData.tokens[id])),
    tokenTemplate(state /*getters*/) {
      return JSON.parse(JSON.stringify(state.tokenData.template));
    },
    tokenListLength(state) {
      return state.tokenData.tokens.length;
    },
    userId(state /*getters*/) {
      return state.userid;
    },
    recaptchaToken(state) {
      return state.recaptchaToken;
    },
  },
  mutations: {
    initTokenData(state, data) {
      state.tokenData = data;
    },
    setUserId(state, userid) {
      state.userid = userid;
    },
    updateTemplate(state, template) {
      state.tokenData.template = JSON.parse(JSON.stringify(template));
    },
    // eslint-disable-next-line no-unused-vars
    updateTemplateLeaf(state, { jsonPath, lastPath, value }) {
      let tmpl = state.tokenData.template.rules;
      jsonPath.forEach((p) => (tmpl = tmpl[p]));
      tmpl[lastPath] = value;
    },
    setRecaptchaToken(state, token) {
      state.recaptchaToken = token;
    },
  },
  actions: {},
  modules: {},
});
