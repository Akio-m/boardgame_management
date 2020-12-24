import { BoardGame, BoardGames, Name, Players } from "@/domain/boardgame";
import BoardgameDriver from "@/driver/boardgameDriver";
import BoardgamePort from "@/port/boardgamePort";

export default class BoardgameGateway implements BoardgamePort {
  constructor(private readonly boardgameDriver: BoardgameDriver) {}
  async getBoardgames(): Promise<BoardGames> {
    const boardgamesJson = await this.boardgameDriver.getBoardgames();
    const list =
      boardgamesJson.boardgames.map(v =>
        new BoardGame(new Name(v.name), new Players(v.players))
      )
    return new BoardGames(list);
  }
}