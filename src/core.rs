#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Player {
  First,
  Second,
}
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum PieceType {
  King, // 王将,玉将
  Rook, // 飛車
  PromotedRook, // 竜王:竜(龍)
  Bishop, // 角行
  PromotedBishop, // 龍馬(竜馬):馬
  GoldGeneral, // 金将
  SilverGeneral, // 銀将
  PromotedSilver, // 成銀:全
  Knight, // 桂馬
  PromotedKnight, // 成桂:圭
  Lance, // 香車
  PromotedLance, // 成香:杏
  Pawn, // 歩兵
  PromotedPawn, // と金
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
