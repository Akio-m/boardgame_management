class Name {
  constructor(readonly value: string) {}
}

class Players {
  constructor(readonly value: string) {}
}
class BoardGame {
  constructor(readonly name: Name, readonly players: Players) {}
}

class BoardGames{
  constructor(readonly values: BoardGame[]) {}
}