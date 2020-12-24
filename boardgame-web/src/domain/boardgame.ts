export class Name {
  constructor(readonly value: string) {}
}

export class Players {
  constructor(readonly value: string) {}
}
export class BoardGame {
  constructor(readonly name: Name, readonly players: Players) {}
}

export class BoardGames{
  constructor(readonly values: BoardGame[]) {}
}