use crate::server::Server;
use statig::prelude::*;

pub enum SessionState {
    Start,
    Login,
    ResourcePack,
    Encryption,
    PreSpawn,
    InGame,
    Death
}

pub struct SessionStateMachine;

impl SessionStateMachine {
    pub fn new() -> Self {
        Self { }
    }
}

#[state_machine(initial = "State::start()")]
impl SessionStateMachine {
    #[state]
    async fn start(event: &SessionState) -> Outcome<State> {
        match event {
            SessionState::Login => Transition(State::login()),
            _ => Super
        }
    }

    #[state]
    async fn login(event: &SessionState) -> Outcome<State> {
        match event {
            SessionState::Encryption => {
                if (Server::get().await.properties.encryption) {
                    Transition(State::encryption())
                } else { Super }
            }
            SessionState::ResourcePack => Transition(State::resource_pack()),
            _ => Super
        }
    }

    #[state]
    async fn encryption(event: &SessionState) -> Outcome<State> {
        match event {
            _ => Super
        }
    }

    #[state]
    async fn resource_pack(event: &SessionState) -> Outcome<State> {
        match event {
            _ => Super
        }
    }
}