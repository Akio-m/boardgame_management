import { BoardGames } from "@/domain/boardgame";
import store from "@/store";

export default class BoardgamePresenter {
  setBoardgames(boardgames: BoardGames) {
    store.dispatch('update', this.boardgamesToJson(boardgames));
  }

  boardgamesToJson(boardgames: BoardGames) {
    return boardgames.values.map(v => {
      return { name: v.name.value, players: v.players.value}
    });
  }
}