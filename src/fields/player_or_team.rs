#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PlayerOrTeam {
    #[default]
    Player,

    Team,
}
