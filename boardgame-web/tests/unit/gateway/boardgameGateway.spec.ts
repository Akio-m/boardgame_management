import BoardgameGateway from "@/gateway/boardgameGateway";

describe("BoardgameGateway", () => {
  test("BoardgamesのJsonをBoardGamesに変換して返す", async () => {
    const boardgameDriverMock = {} as BoardgameDriver;
    const boardGameGateway = new BoardgameGateway(boardgameDriverMock);

    const boardgameJson = {
      boardgames: [
        {
          name: "boardgame1",
          players: "1 - 2"
        },
        {
          name: "boardgame2",
          players: "3 - 4"
        },
        {
          name: "boardgame3",
          players: "5 - "
        }
      ]
    }

    const getBoardGamesFn = jest.fn();
    getBoardGamesFn.mockReturnValueOnce(boardgameJson)
    boardgameDriverMock.getBoardgames = getBoardGamesFn;

    const actual = await boardGameGateway.getBoardgames();

    const expected = new BoardGames([
      new BoardGame(new Name("boardgame1"), new Players("1 - 2")),
      new BoardGame(new Name("boardgame2"), new Players("3 - 4")),
      new BoardGame(new Name("boardgame3"), new Players("5 -"))
    ]);

    expect(actual).toEqual(expected);
    expect(boardgameDriverMock.getBoardgames).toHaveBeenCalled();
  });
});
