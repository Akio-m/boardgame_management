import axios from "axios";

export type BoardGameJson = { name: string, players: string }

export type BoardGamesJson = { boardgames: BoardGameJson[] }

export default class BoardgameDriver {
  async getBoardgames(): Promise<BoardGamesJson> {
    return await (await axios.get(`http://localhost:21001/v1/boardgames`)).data;
  }
}