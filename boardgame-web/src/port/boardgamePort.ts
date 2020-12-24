import { BoardGames } from "@/domain/boardgame";

export default interface BoardgamePort {
  getBoardgames(): Promise<BoardGames>;
}