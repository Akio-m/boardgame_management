import { BoardGames } from "@/domain/boardgame";
import Vue from "vue";
import Vuex from "vuex";

Vue.use(Vuex);

export default new Vuex.Store({
  state: {
    boardgames: []
  },
  mutations: {
    update (state, boardgames) {
      state.boardgames = boardgames;
    }
  },
  actions: {
    update (context, boadgames) {
      context.commit('update', boadgames);
    }
  }
});
