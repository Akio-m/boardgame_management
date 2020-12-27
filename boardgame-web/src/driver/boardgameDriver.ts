import axios from "axios";

export type BoardGameJson = { name: string, players: string }

export type BoardGamesJson = { boardgames: BoardGameJson[] }

export default class BoardgameDriver {
  async getBoardgames(): Promise<BoardGamesJson> {
    const port = 21001;
    const url = `http://localhost:${port}`
    return await (await axios.get(`${url}/v1/boardgames`)).data;
  }
}