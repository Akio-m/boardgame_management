<template>
  <v-container class="grey lighten-5">
    <h1>BoardGame List</h1>
    <v-simple-table>
      <template v-slot:default>
        <thead>
          <tr class="blue lighten-5">
            <th class="text-center">ボードゲーム名</th>
            <th class="text-center">プレイ可能人数(人)</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="boardgame in boardgames" :key="boardgame.name" class="text-center">
            <td>{{ boardgame.name }}</td>
            <td>{{ boardgame.players }}</td>
          </tr>
        </tbody>
      </template>
    </v-simple-table>
  </v-container>
</template>

<script lang="ts">
import BoardgameDriver from "@/driver/boardgameDriver";
import BoardgameGateway from "@/gateway/boardgameGateway";
import BoardgamePresenter from "@/presenter/boardgamePresenter";
import BoardgameUsecase from "@/usecase/boardgameUsecase";
import { Component, Prop, Vue } from "vue-property-decorator";

@Component
export default class BoardGame extends Vue {
  data () {
    return {
      boardgames: this.$store.state.boardgames
    }
  }
  created() {
    // TODO: injectするようにする
    const presenter = new BoardgamePresenter();
    const driver = new BoardgameDriver();
    const port = new BoardgameGateway(driver);
    const usecase = new BoardgameUsecase(port, presenter);
    usecase.getBoardGames();
  }
  // ここらへんでデータを読み込む

}
</script>

<style scoped lang="scss">
</style>
