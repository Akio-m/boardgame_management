import BoardgamePort from "@/port/boardgamePort";

export default class BoardgameUsecase {
  constructor(
    readonly boardgamePort: BoardgamePort,
    readonly boardgamePresenter: BoardgamePresenter) {}

  async getBoardGames(): Promise<void> {
    const boardgames = await this.boardgamePort.getBoardgames();
    this.boardgamePresenter.setBoardgames(boardgames);
  }
}