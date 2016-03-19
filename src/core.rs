#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Player {
    /// 先手
    First,
    /// 後手
    Second,
}
#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub enum PieceType {
    /// 王将,玉将
    King,
    /// 飛車
    Rook,
    /// 竜王:竜(龍)
    PromotedRook,
    /// 角行
    Bishop,
    /// 龍馬(竜馬):馬
    PromotedBishop,
    /// 金将
    GoldGeneral,
    /// 銀将
    SilverGeneral,
    /// 成銀:全
    PromotedSilver,
    /// 桂馬
    Knight,
    /// 成桂:圭
    PromotedKnight,
    /// 香車
    Lance,
    /// 成香:杏
    PromotedLance,
    /// 歩兵
    Pawn,
    /// と金
    PromotedPawn,
}
impl PieceType {
    pub fn promote(&self) -> Option<PieceType> {
        match *self {
            PieceType::Rook => Some(PieceType::PromotedRook),
            PieceType::Bishop => Some(PieceType::PromotedBishop),
            PieceType::SilverGeneral => Some(PieceType::PromotedSilver),
            PieceType::Knight => Some(PieceType::PromotedKnight),
            PieceType::Lance => Some(PieceType::PromotedLance),
            PieceType::Pawn => Some(PieceType::PromotedPawn),
            _ => None,
        }
    }
}
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Cell {
    Space(usize),
    Piece((PieceType, Player)),
}
