import { BoardGame, BoardGames, Name, Players } from "@/domain/boardgame";
import BoardgameDriver from "@/driver/boardgameDriver";
import BoardgameGateway from "@/gateway/boardgameGateway";

describe("BoardgameGateway", () => {
  test("BoardgamesのJsonをBoardGamesに変換して返す", async () => {
    const boardgameDriverMock = {} as BoardgameDriver;
    const boardGameGateway = new BoardgameGateway(boardgameDriverMock);

    const boardgameJson = {
      boardgames: [
        {
          name: "boardgame1",
          name_kana: "",
          players_min: "1",
          players_max: "2",
          play_time_min: "",
          play_time_max: "",
          ages: "",
          manufacturer: ""
        },
        {
          name: "boardgame2",
          name_kana: "",
          players_min: "3",
          players_max: "4",
          play_time_min: "",
          play_time_max: "",
          ages: "",
          manufacturer: ""
        },
        {
          name: "boardgame3",
          name_kana: "",
          players_min: "5",
          players_max: "",
          play_time_min: "",
          play_time_max: "",
          ages: "",
          manufacturer: ""
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
      new BoardGame(new Name("boardgame3"), new Players("5 - "))
    ]);

    expect(actual).toEqual(expected);
    expect(boardgameDriverMock.getBoardgames).toHaveBeenCalled();
  });
});
