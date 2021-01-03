import axios from "axios";

export type BoardGameJson = {
  name: string,
  name_kana: string,
  players_min: string,
  players_max: string,
  play_time_min: string,
  play_time_max: string,
  ages: string,
  manufacturer: string
}

export type BoardGamesJson = { boardgames: BoardGameJson[] }

export default class BoardgameDriver {
  async getBoardgames(): Promise<BoardGamesJson> {
    const port = 21001;
    const url = `http://localhost:${port}`
    return await (await axios.get(`${url}/v1/boardgames`)).data;
  }
}