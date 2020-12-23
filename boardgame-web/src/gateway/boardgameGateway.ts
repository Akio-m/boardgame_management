import BoardgamePort from "@/port/boardgamePort";

export default class BoardgameGateway implements BoardgamePort {
  constructor(readonly boardgameDriver: BoardgameDriver) {}
  getBoardgames(): Promise<BoardGames> {
    throw new Error("Method not implemented.");
  }
}