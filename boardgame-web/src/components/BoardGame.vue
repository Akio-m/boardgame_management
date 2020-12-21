<template>
  <div class="hello">
    <h1>BoardGame List</h1>
    <table class="boardgame-table">
      <thead>
        <tr>
          <th>ボードゲーム名</th>
          <th>プレイ可能人数(人)</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td>カタン スタンダード版</td>
          <td>3 - 4</td>
        </tr>
        <tr>
          <td>ニュクトフォビア</td>
          <td>3 - 5</td>
        </tr>
        <tr>
          <td>裏切りの工作員</td>
          <td>3 - 9</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script lang="ts">
import { Component, Prop, Vue } from "vue-property-decorator";

@Component
export default class BoardGame extends Vue {
  //@Prop() private msg!: string;
//   setup() {

//   }
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
