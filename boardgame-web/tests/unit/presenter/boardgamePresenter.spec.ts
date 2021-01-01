import { BoardGame, BoardGames, Name, Players } from "@/domain/boardgame";
import BoardgamePresenter from "@/presenter/boardgamePresenter";

describe("BoardgamePresenter", () => {
  test("boardgamesをjsonに変換する", () => {
    const boardgamePresenter = new BoardgamePresenter();

    const boardgames = new BoardGames([
      new BoardGame(new Name("boardgame1"), new Players("1 - 2")),
      new BoardGame(new Name("boardgame2"), new Players("3 - 4")),
      new BoardGame(new Name("boardgame3"), new Players("5 - "))
    ]);

    const expected = [
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
    ];

    expect(boardgamePresenter.boardgamesToJson(boardgames)).toEqual(expected);
  });
});
