<template>
  <v-container class="grey lighten-5">
    <h1>BoardGame List</h1>
    <v-simple-table>
      <template v-slot:default>
        <thead>
          <tr>
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
import { Component, Prop, Vue } from "vue-property-decorator";

@Component
export default class BoardGame extends Vue {
  data () {
    return {
      boardgames: [
        {
          name: "カタン スタンダード版",
          players: "3 - 4"
        },
        {
          name: "ニュクトフォビア",
          players: "3 - 5"
        },
        {
          name: "裏切りの工作員",
          players: "3 - 9"
        }
      ]
    }
  }
}
</script>

<style scoped lang="scss">
h3 {
  margin: 40px 0 0;
}
ul {
  list-style-type: none;
  padding: 0;
}
li {
  display: inline-block;
  margin: 0 10px;
}
a {
  color: #42b983;
}

.boardgame-table {
  width: 100%;
  border-collapse: collapse;
  text-align: center;
  border-top: 1px solid #ccc;
  th {
    padding: 10px;
    border-bottom: 1px solid #ccc;
    background-color: #42b983;
    color: #ffffff;
  }
  td {
    padding: 10px;
    border-bottom: 1px solid #ccc;
  }
}
</style>
