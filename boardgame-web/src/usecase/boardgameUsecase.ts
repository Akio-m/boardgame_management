export default class BoardgameUsecase {
  constructor(
    readonly boardgamePort: BoardgamePort,
    readonly boardgamePresenter: BoardgamePresenter) {}

  async findAll(): Promise<void> {
    const boardgames = this.boardgamePort.findAll();
    this.boardgamePresenter.setBoardgame(boardgames);
  }
}