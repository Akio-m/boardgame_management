import { BoardGames } from "@/domain/boardgame";
import BoardgamePort from "@/port/boardgamePort";
import BoardgamePresenter from "@/presenter/boardgamePresenter";
import BoardgameUsecase from "@/usecase/boardgameUsecase";

describe("BoardgameUsecase", () => {
  test("ボードゲームの一覧を取得しPresenterに渡す", async () => {
    const boardgamePortMock = {} as BoardgamePort;
    const boardgamePresenterMock = {} as BoardgamePresenter;
    const boardGameUsecase = new BoardgameUsecase(boardgamePortMock, boardgamePresenterMock);

    const boardgames = {} as BoardGames;
    const getBoardGamesFn = jest.fn();
    getBoardGamesFn.mockReturnValueOnce(boardgames)
    boardgamePortMock.getBoardgames = getBoardGamesFn;

    const setBoardgamesFn = jest.fn();
    boardgamePresenterMock.setBoardgames = setBoardgamesFn;

    await boardGameUsecase.getBoardGames();

    expect(boardgamePortMock.getBoardgames).toHaveBeenCalled();
    expect(boardgamePresenterMock.setBoardgames).toHaveBeenCalledWith(boardgames);
  });
});
