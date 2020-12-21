<template>
  <v-app>
    <v-main>
      <BoardGame/>
    </v-main>å
  </v-app>
</template>

<script lang="ts">
import Vue from "vue";
import BoardGame from "./components/BoardGame.vue";

export default Vue.extend({
  name: "App",

  components: {
    BoardGame
  },

  data: () => ({
    //
  })
});
</script>
