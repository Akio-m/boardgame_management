table! {
  boardgame(name) {
    name -> VarChar,
    name_kana -> VarChar,
    players_min -> Nullable<Integer>,
    players_max -> Nullable<Integer>,
    play_time_min -> Nullable<Integer>,
    play_time_max -> Nullable<Integer>,
    age -> Nullable<Integer>,
    manufacturer -> Nullable<VarChar>,
  }
}
