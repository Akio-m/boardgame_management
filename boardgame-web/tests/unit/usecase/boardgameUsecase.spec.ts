import BoardgameUsecase from "@/usecase/boardgameUsecase";

describe("BoardgameUsecase", () => {
  test("ボードゲームの一覧を取得しPresenterに渡す", async () => {
    const boardgamePortMock = {} as BoardgamePort;
    const boardgamePresenterMock = {} as BoardgamePresenter;
    const boardGameUsecase = new BoardgameUsecase(boardgamePortMock, boardgamePresenterMock);

    const boardgames = {} as BoardGames;
    const findAllFn = jest.fn();
    findAllFn.mockReturnValueOnce(boardgames)
    boardgamePortMock.findAll = findAllFn;

    const setBoardgameFn = jest.fn();
    boardgamePresenterMock.setBoardgame = setBoardgameFn;

    await boardGameUsecase.findAll();

    expect(boardgamePortMock.findAll).toHaveBeenCalled();
    expect(boardgamePresenterMock.setBoardgame).toHaveBeenCalledWith(boardgames);
  });
});
