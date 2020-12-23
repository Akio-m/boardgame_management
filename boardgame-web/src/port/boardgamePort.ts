export default interface BoardgamePort {
  getBoardgames(): Promise<BoardGames>;
}