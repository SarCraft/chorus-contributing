#[derive(PartialEq, Eq, Clone, Debug)]
pub enum SessionState {
    Start,
    Login,
    ResourcePack,
    Encryption,
    PreSpawn,
    InGame,
    Death
}