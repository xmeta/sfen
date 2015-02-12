#![allow(non_snake_case, unused)]
mod core;
pub use core::{PieceType, Player, Cell};
use std::collections::VecMap;
use self::ParseResult::{Matched, Failed};
enum ParseResult<T> { Matched(usize, T), Failed, }
struct ParseState {
    max_err_pos: usize,
    expected: ::std::collections::HashSet<&'static str>,
}
impl ParseState {
    fn new() -> ParseState {
        ParseState{max_err_pos: 0,
                   expected: ::std::collections::HashSet::new(),}
    }
    fn mark_failure(&mut self, pos: usize, expected: &'static str)
     -> ParseResult<()> {
        if pos > self.max_err_pos {
            self.max_err_pos = pos;
            self.expected.clear();
        }
        if pos == self.max_err_pos { self.expected.insert(expected); }
        Failed
    }
}
fn slice_eq(input: &str, state: &mut ParseState, pos: usize, m: &'static str)
 -> ParseResult<()> {
    #![inline]
    #![allow(dead_code)]
    let l = m.len();
    if input.len() >= pos + l &&
           &input.as_bytes()[pos..pos + l] == m.as_bytes() {
        Matched(pos + l, ())
    } else { state.mark_failure(pos, m) }
}
fn slice_eq_case_insensitive(input: &str, state: &mut ParseState, pos: usize,
                             m: &'static str) -> ParseResult<()> {
    #![inline]
    #![allow(dead_code)]
    let mut used = 0us;
    let mut input_iter = input[pos..].chars();
    for m_char in m.chars() {
        let m_char_upper = m_char.to_uppercase();
        used += m_char_upper.len_utf8();
        let input_char_result = input_iter.next();
        if input_char_result.is_none() ||
               input_char_result.unwrap().to_uppercase() != m_char_upper {
            return state.mark_failure(pos, m);
        }
    }
    Matched(pos + used, ())
}
fn any_char(input: &str, state: &mut ParseState, pos: usize)
 -> ParseResult<()> {
    #![inline]
    #![allow(dead_code)]
    if input.len() > pos {
        Matched(input.char_range_at(pos).next, ())
    } else { state.mark_failure(pos, "<character>") }
}
fn pos_to_line(input: &str, pos: usize) -> (usize, usize) {
    let mut remaining = pos;
    let mut lineno: usize = 1;
    for line in input.lines() {
        let line_length = line.len() + 1;
        if remaining < line_length { return (lineno, remaining + 1); }
        remaining -= line_length;
        lineno += 1;
    }
    return (lineno, remaining + 1);
}
fn parse_root<'input>(input: &'input str, state: &mut ParseState, pos: usize)
 ->
     ParseResult<((VecMap<(PieceType, Player)>, usize, usize), Player,
                  Option<Vec<(PieceType, Player)>>, Option<usize>)> {
    {
        let start_pos = pos;
        {
            let seq_res = parse_board(input, state, pos);
            match seq_res {
                Matched(pos, b) => {
                    {
                        let seq_res =
                            {
                                let mut repeat_pos = pos;
                                let mut repeat_value = vec!();
                                loop  {
                                    let pos = repeat_pos;
                                    let step_res =
                                        slice_eq(input, state, pos, " ");
                                    match step_res {
                                        Matched(newpos, value) => {
                                            repeat_pos = newpos;
                                            repeat_value.push(value);
                                        }
                                        Failed => { break ; }
                                    }
                                }
                                if repeat_value.len() >= 1us {
                                    Matched(repeat_pos, ())
                                } else { Failed }
                            };
                        match seq_res {
                            Matched(pos, _) => {
                                {
                                    let seq_res =
                                        parse_player(input, state, pos);
                                    match seq_res {
                                        Matched(pos, p) => {
                                            {
                                                let seq_res =
                                                    {
                                                        let mut repeat_pos =
                                                            pos;
                                                        let mut repeat_value =
                                                            vec!();
                                                        loop  {
                                                            let pos =
                                                                repeat_pos;
                                                            let step_res =
                                                                slice_eq(input,
                                                                         state,
                                                                         pos,
                                                                         " ");
                                                            match step_res {
                                                                Matched(newpos,
                                                                        value)
                                                                => {
                                                                    repeat_pos
                                                                        =
                                                                        newpos;
                                                                    repeat_value.push(value);
                                                                }
                                                                Failed => {
                                                                    break ;
                                                                }
                                                            }
                                                        }
                                                        if repeat_value.len()
                                                               >= 1us {
                                                            Matched(repeat_pos,
                                                                    ())
                                                        } else { Failed }
                                                    };
                                                match seq_res {
                                                    Matched(pos, _) => {
                                                        {
                                                            let seq_res =
                                                                parse_captured_pieces(input,
                                                                                      state,
                                                                                      pos);
                                                            match seq_res {
                                                                Matched(pos,
                                                                        cap)
                                                                => {
                                                                    {
                                                                        let seq_res =
                                                                            match {
                                                                                      let start_pos =
                                                                                          pos;
                                                                                      {
                                                                                          let seq_res =
                                                                                              {
                                                                                                  let mut repeat_pos =
                                                                                                      pos;
                                                                                                  let mut repeat_value =
                                                                                                      vec!();
                                                                                                  loop
                                                                                                       {
                                                                                                      let pos =
                                                                                                          repeat_pos;
                                                                                                      let step_res =
                                                                                                          slice_eq(input,
                                                                                                                   state,
                                                                                                                   pos,
                                                                                                                   " ");
                                                                                                      match step_res
                                                                                                          {
                                                                                                          Matched(newpos,
                                                                                                                  value)
                                                                                                          =>
                                                                                                          {
                                                                                                              repeat_pos
                                                                                                                  =
                                                                                                                  newpos;
                                                                                                              repeat_value.push(value);
                                                                                                          }
                                                                                                          Failed
                                                                                                          =>
                                                                                                          {
                                                                                                              break
                                                                                                                  ;
                                                                                                          }
                                                                                                      }
                                                                                                  }
                                                                                                  if repeat_value.len()
                                                                                                         >=
                                                                                                         1us
                                                                                                     {
                                                                                                      Matched(repeat_pos,
                                                                                                              ())
                                                                                                  } else {
                                                                                                      Failed
                                                                                                  }
                                                                                              };
                                                                                          match seq_res
                                                                                              {
                                                                                              Matched(pos,
                                                                                                      _)
                                                                                              =>
                                                                                              {
                                                                                                  {
                                                                                                      let seq_res =
                                                                                                          parse_num(input,
                                                                                                                    state,
                                                                                                                    pos);
                                                                                                      match seq_res
                                                                                                          {
                                                                                                          Matched(pos,
                                                                                                                  n)
                                                                                                          =>
                                                                                                          {
                                                                                                              {
                                                                                                                  let match_str =
                                                                                                                      &input[start_pos..pos];
                                                                                                                  Matched(pos,
                                                                                                                          {
                                                                                                                              n
                                                                                                                          })
                                                                                                              }
                                                                                                          }
                                                                                                          Failed
                                                                                                          =>
                                                                                                          Failed,
                                                                                                      }
                                                                                                  }
                                                                                              }
                                                                                              Failed
                                                                                              =>
                                                                                              Failed,
                                                                                          }
                                                                                      }
                                                                                  }
                                                                                {
                                                                                Matched(newpos,
                                                                                        value)
                                                                                =>
                                                                                {
                                                                                    Matched(newpos,
                                                                                            Some(value))
                                                                                }
                                                                                Failed
                                                                                =>
                                                                                {
                                                                                    Matched(pos,
                                                                                            None)
                                                                                }
                                                                            };
                                                                        match seq_res
                                                                            {
                                                                            Matched(pos,
                                                                                    n)
                                                                            =>
                                                                            {
                                                                                {
                                                                                    let match_str =
                                                                                        &input[start_pos..pos];
                                                                                    Matched(pos,
                                                                                            {
                                                                                                (b,
                                                                                                 p,
                                                                                                 cap,
                                                                                                 n)
                                                                                            })
                                                                                }
                                                                            }
                                                                            Failed
                                                                            =>
                                                                            Failed,
                                                                        }
                                                                    }
                                                                }
                                                                Failed =>
                                                                Failed,
                                                            }
                                                        }
                                                    }
                                                    Failed => Failed,
                                                }
                                            }
                                        }
                                        Failed => Failed,
                                    }
                                }
                            }
                            Failed => Failed,
                        }
                    }
                }
                Failed => Failed,
            }
        }
    }
}
fn parse_player<'input>(input: &'input str, state: &mut ParseState,
                        pos: usize) -> ParseResult<Player> {
    {
        let choice_res =
            {
                let start_pos = pos;
                {
                    let seq_res = slice_eq(input, state, pos, "b");
                    match seq_res {
                        Matched(pos, _) => {
                            {
                                let match_str = &input[start_pos..pos];
                                Matched(pos, { Player::First })
                            }
                        }
                        Failed => Failed,
                    }
                }
            };
        match choice_res {
            Matched(pos, value) => Matched(pos, value),
            Failed => {
                let start_pos = pos;
                {
                    let seq_res = slice_eq(input, state, pos, "w");
                    match seq_res {
                        Matched(pos, _) => {
                            {
                                let match_str = &input[start_pos..pos];
                                Matched(pos, { Player::Second })
                            }
                        }
                        Failed => Failed,
                    }
                }
            }
        }
    }
}
fn parse_captured_pieces<'input>(input: &'input str, state: &mut ParseState,
                                 pos: usize)
 -> ParseResult<Option<Vec<(PieceType, Player)>>> {
    {
        let choice_res =
            {
                let start_pos = pos;
                {
                    let seq_res = slice_eq(input, state, pos, "-");
                    match seq_res {
                        Matched(pos, _) => {
                            {
                                let match_str = &input[start_pos..pos];
                                Matched(pos, { None })
                            }
                        }
                        Failed => Failed,
                    }
                }
            };
        match choice_res {
            Matched(pos, value) => Matched(pos, value),
            Failed => {
                let start_pos = pos;
                {
                    let seq_res =
                        {
                            let mut repeat_pos = pos;
                            let mut repeat_value = vec!();
                            loop  {
                                let pos = repeat_pos;
                                let step_res =
                                    {
                                        let start_pos = pos;
                                        {
                                            let seq_res =
                                                match parse_num(input, state,
                                                                pos) {
                                                    Matched(newpos, value) =>
                                                    {
                                                        Matched(newpos,
                                                                Some(value))
                                                    }
                                                    Failed => {
                                                        Matched(pos, None)
                                                    }
                                                };
                                            match seq_res {
                                                Matched(pos, count) => {
                                                    {
                                                        let seq_res =
                                                            {
                                                                let choice_res =
                                                                    parse_non_promotable_piece(input,
                                                                                               state,
                                                                                               pos);
                                                                match choice_res
                                                                    {
                                                                    Matched(pos,
                                                                            value)
                                                                    =>
                                                                    Matched(pos,
                                                                            value),
                                                                    Failed =>
                                                                    parse_promotable_piece(input,
                                                                                           state,
                                                                                           pos),
                                                                }
                                                            };
                                                        match seq_res {
                                                            Matched(pos,
                                                                    types) =>
                                                            {
                                                                {
                                                                    let match_str =
                                                                        &input[start_pos..pos];
                                                                    Matched(pos,
                                                                            {
                                                                                match count
                                                                                    {
                                                                                    Some(x)
                                                                                    =>
                                                                                    (x,
                                                                                     types),
                                                                                    None
                                                                                    =>
                                                                                    (1,
                                                                                     types),
                                                                                }
                                                                            })
                                                                }
                                                            }
                                                            Failed => Failed,
                                                        }
                                                    }
                                                }
                                                Failed => Failed,
                                            }
                                        }
                                    };
                                match step_res {
                                    Matched(newpos, value) => {
                                        repeat_pos = newpos;
                                        repeat_value.push(value);
                                    }
                                    Failed => { break ; }
                                }
                            }
                            if repeat_value.len() >= 1us {
                                Matched(repeat_pos, repeat_value)
                            } else { Failed }
                        };
                    match seq_res {
                        Matched(pos, koma) => {
                            {
                                let match_str = &input[start_pos..pos];
                                Matched(pos,
                                        {
                                            let mut vec = Vec::new();
                                            for k in koma.iter() {
                                                for i in range(0, k.0) {
                                                    vec.push(k.1);
                                                }
                                            }
                                            Some(vec)
                                        })
                            }
                        }
                        Failed => Failed,
                    }
                }
            }
        }
    }
}
fn parse_board<'input>(input: &'input str, state: &mut ParseState, pos: usize)
 -> ParseResult<(VecMap<(PieceType, Player)>, usize, usize)> {
    {
        let start_pos = pos;
        {
            let seq_res = parse_row(input, state, pos);
            match seq_res {
                Matched(pos, l) => {
                    {
                        let seq_res =
                            {
                                let mut repeat_pos = pos;
                                let mut repeat_value = vec!();
                                loop  {
                                    let pos = repeat_pos;
                                    let step_res =
                                        {
                                            let start_pos = pos;
                                            {
                                                let seq_res =
                                                    if input.len() > pos {
                                                        let ::std::str::CharRange {
                                                                ch, next } =
                                                            input.char_range_at(pos);
                                                        match ch {
                                                            '/' =>
                                                            Matched(next, ()),
                                                            _ =>
                                                            state.mark_failure(pos,
                                                                               "[/]"),
                                                        }
                                                    } else {
                                                        state.mark_failure(pos,
                                                                           "[/]")
                                                    };
                                                match seq_res {
                                                    Matched(pos, _) => {
                                                        {
                                                            let seq_res =
                                                                parse_row(input,
                                                                          state,
                                                                          pos);
                                                            match seq_res {
                                                                Matched(pos,
                                                                        r) =>
                                                                {
                                                                    {
                                                                        let match_str =
                                                                            &input[start_pos..pos];
                                                                        Matched(pos,
                                                                                {
                                                                                    r
                                                                                })
                                                                    }
                                                                }
                                                                Failed =>
                                                                Failed,
                                                            }
                                                        }
                                                    }
                                                    Failed => Failed,
                                                }
                                            }
                                        };
                                    match step_res {
                                        Matched(newpos, value) => {
                                            repeat_pos = newpos;
                                            repeat_value.push(value);
                                        }
                                        Failed => { break ; }
                                    }
                                }
                                Matched(repeat_pos, repeat_value)
                            };
                        match seq_res {
                            Matched(pos, r) => {
                                {
                                    let match_str = &input[start_pos..pos];
                                    Matched(pos,
                                            {
                                                let mut vec = Vec::new();
                                                vec.push(l.0);
                                                let cols: usize = l.1;
                                                for c in r.into_iter() {
                                                    if c.1 == cols {
                                                        vec.push(c.0);
                                                    } else {
                                                        panic!("The number of cols on the board is {}, but found {} cols"
                                                               , cols , c .
                                                               1);
                                                    }
                                                }
                                                let mut vmap = VecMap::new();
                                                for (i, r) in
                                                    vec.iter().enumerate() {
                                                    for (key, value) in
                                                        r.iter() {
                                                        let index =
                                                            i * cols + key;
                                                        vmap.insert(index,
                                                                    *value);
                                                    }
                                                }
                                                (vmap, cols, vec.len())
                                            })
                                }
                            }
                            Failed => Failed,
                        }
                    }
                }
                Failed => Failed,
            }
        }
    }
}
fn parse_row<'input>(input: &'input str, state: &mut ParseState, pos: usize)
 -> ParseResult<(VecMap<(PieceType, Player)>, usize)> {
    {
        let start_pos = pos;
        {
            let seq_res =
                {
                    let mut repeat_pos = pos;
                    let mut repeat_value = vec!();
                    loop  {
                        let pos = repeat_pos;
                        let step_res =
                            {
                                let choice_res =
                                    parse_space(input, state, pos);
                                match choice_res {
                                    Matched(pos, value) =>
                                    Matched(pos, value),
                                    Failed =>
                                    parse_koma_type(input, state, pos),
                                }
                            };
                        match step_res {
                            Matched(newpos, value) => {
                                repeat_pos = newpos;
                                repeat_value.push(value);
                            }
                            Failed => { break ; }
                        }
                    }
                    Matched(repeat_pos, repeat_value)
                };
            match seq_res {
                Matched(pos, cell) => {
                    {
                        let match_str = &input[start_pos..pos];
                        Matched(pos,
                                {
                                    let mut vmap = VecMap::new();
                                    let mut count = 0;
                                    for c in cell.iter() {
                                        match *c {
                                            Cell::Space(x) => {
                                                count += x;
                                                continue
                                            }
                                            Cell::Piece(x) => {
                                                vmap.insert(count, x);
                                            }
                                        }
                                        count += 1;
                                    }
                                    (vmap, count)
                                })
                    }
                }
                Failed => Failed,
            }
        }
    }
}
fn parse_koma_type<'input>(input: &'input str, state: &mut ParseState,
                           pos: usize) -> ParseResult<Cell> {
    {
        let choice_res =
            {
                let start_pos = pos;
                {
                    let seq_res =
                        match if input.len() > pos {
                                  let ::std::str::CharRange { ch, next } =
                                      input.char_range_at(pos);
                                  match ch {
                                      '+' => Matched(next, ()),
                                      _ => state.mark_failure(pos, "[+]"),
                                  }
                              } else { state.mark_failure(pos, "[+]") } {
                            Matched(newpos, value) => {
                                Matched(newpos, Some(value))
                            }
                            Failed => { Matched(pos, None) }
                        };
                    match seq_res {
                        Matched(pos, t) => {
                            {
                                let seq_res =
                                    parse_promotable_piece(input, state, pos);
                                match seq_res {
                                    Matched(pos, k) => {
                                        {
                                            let match_str =
                                                &input[start_pos..pos];
                                            Matched(pos,
                                                    {
                                                        let koma =
                                                            if t != None {
                                                                let ktype:
                                                                        PieceType =
                                                                    k.0;
                                                                (ktype.promote().unwrap(),
                                                                 k.1)
                                                            } else { k };
                                                        Cell::Piece(koma)
                                                    })
                                        }
                                    }
                                    Failed => Failed,
                                }
                            }
                        }
                        Failed => Failed,
                    }
                }
            };
        match choice_res {
            Matched(pos, value) => Matched(pos, value),
            Failed => {
                let start_pos = pos;
                {
                    let seq_res =
                        parse_non_promotable_piece(input, state, pos);
                    match seq_res {
                        Matched(pos, k) => {
                            {
                                let match_str = &input[start_pos..pos];
                                Matched(pos, { Cell::Piece(k) })
                            }
                        }
                        Failed => Failed,
                    }
                }
            }
        }
    }
}
fn parse_non_promotable_piece<'input>(input: &'input str,
                                      state: &mut ParseState, pos: usize)
 -> ParseResult<(PieceType, Player)> {
    {
        let start_pos = pos;
        {
            let seq_res =
                if input.len() > pos {
                    let ::std::str::CharRange { ch, next } =
                        input.char_range_at(pos);
                    match ch {
                        'G' | 'g' | 'K' | 'k' => Matched(next, ()),
                        _ => state.mark_failure(pos, "[GgKk]"),
                    }
                } else { state.mark_failure(pos, "[GgKk]") };
            match seq_res {
                Matched(pos, _) => {
                    {
                        let match_str = &input[start_pos..pos];
                        Matched(pos,
                                {
                                    match match_str {
                                        "G" =>
                                        (PieceType::GoldGeneral,
                                         Player::First),
                                        "g" =>
                                        (PieceType::GoldGeneral,
                                         Player::Second),
                                        "K" =>
                                        (PieceType::King, Player::First),
                                        "k" =>
                                        (PieceType::King, Player::Second),
                                        _ => { panic!("syntax error"); }
                                    }
                                })
                    }
                }
                Failed => Failed,
            }
        }
    }
}
fn parse_promotable_piece<'input>(input: &'input str, state: &mut ParseState,
                                  pos: usize)
 -> ParseResult<(PieceType, Player)> {
    {
        let start_pos = pos;
        {
            let seq_res =
                if input.len() > pos {
                    let ::std::str::CharRange { ch, next } =
                        input.char_range_at(pos);
                    match ch {
                        'P' | 'L' | 'N' | 'S' | 'B' | 'R' | 'p' | 'l' | 'n' |
                        's' | 'b' | 'r' => Matched(next, ()),
                        _ => state.mark_failure(pos, "[PLNSBRplnsbr]"),
                    }
                } else { state.mark_failure(pos, "[PLNSBRplnsbr]") };
            match seq_res {
                Matched(pos, _) => {
                    {
                        let match_str = &input[start_pos..pos];
                        Matched(pos,
                                {
                                    match match_str {
                                        "P" =>
                                        (PieceType::Pawn, Player::First),
                                        "L" =>
                                        (PieceType::Lance, Player::First),
                                        "N" =>
                                        (PieceType::Knight, Player::First),
                                        "S" =>
                                        (PieceType::SilverGeneral,
                                         Player::First),
                                        "R" =>
                                        (PieceType::Rook, Player::First),
                                        "B" =>
                                        (PieceType::Bishop, Player::First),
                                        "p" =>
                                        (PieceType::Pawn, Player::Second),
                                        "l" =>
                                        (PieceType::Lance, Player::Second),
                                        "n" =>
                                        (PieceType::Knight, Player::Second),
                                        "s" =>
                                        (PieceType::SilverGeneral,
                                         Player::Second),
                                        "r" =>
                                        (PieceType::Rook, Player::Second),
                                        "b" =>
                                        (PieceType::Bishop, Player::Second),
                                        _ => {
                                            panic!(match_str . to_string (  ))
                                        }
                                    }
                                })
                    }
                }
                Failed => Failed,
            }
        }
    }
}
fn parse_space<'input>(input: &'input str, state: &mut ParseState, pos: usize)
 -> ParseResult<Cell> {
    {
        let start_pos = pos;
        {
            let seq_res = parse_num(input, state, pos);
            match seq_res {
                Matched(pos, n) => {
                    {
                        let match_str = &input[start_pos..pos];
                        Matched(pos, { Cell::Space(n) })
                    }
                }
                Failed => Failed,
            }
        }
    }
}
fn parse_num<'input>(input: &'input str, state: &mut ParseState, pos: usize)
 -> ParseResult<usize> {
    {
        let start_pos = pos;
        {
            let seq_res =
                {
                    let mut repeat_pos = pos;
                    let mut repeat_value = vec!();
                    loop  {
                        let pos = repeat_pos;
                        let step_res =
                            if input.len() > pos {
                                let ::std::str::CharRange { ch, next } =
                                    input.char_range_at(pos);
                                match ch {
                                    '0' ...'9' => Matched(next, ()),
                                    _ => state.mark_failure(pos, "[0-9]"),
                                }
                            } else { state.mark_failure(pos, "[0-9]") };
                        match step_res {
                            Matched(newpos, value) => {
                                repeat_pos = newpos;
                                repeat_value.push(value);
                            }
                            Failed => { break ; }
                        }
                    }
                    if repeat_value.len() >= 1us {
                        Matched(repeat_pos, ())
                    } else { Failed }
                };
            match seq_res {
                Matched(pos, _) => {
                    {
                        let match_str = &input[start_pos..pos];
                        Matched(pos, { match_str.parse().unwrap() })
                    }
                }
                Failed => Failed,
            }
        }
    }
}
pub fn root<'input>(input: &'input str)
 ->
     Result<((VecMap<(PieceType, Player)>, usize, usize), Player,
             Option<Vec<(PieceType, Player)>>, Option<usize>), String> {
    let mut state = ParseState::new();
    match parse_root(input, &mut state, 0) {
        Matched(pos, value) => { if pos == input.len() { return Ok(value) } }
        _ => { }
    }
    let (line, col) = pos_to_line(input, state.max_err_pos);
    Err(format!("Error at Line {}:{}: Expected {:?}" , line , col , state .
                expected))
}
pub fn player<'input>(input: &'input str) -> Result<Player, String> {
    let mut state = ParseState::new();
    match parse_player(input, &mut state, 0) {
        Matched(pos, value) => { if pos == input.len() { return Ok(value) } }
        _ => { }
    }
    let (line, col) = pos_to_line(input, state.max_err_pos);
    Err(format!("Error at Line {}:{}: Expected {:?}" , line , col , state .
                expected))
}
pub fn captured_pieces<'input>(input: &'input str)
 -> Result<Option<Vec<(PieceType, Player)>>, String> {
    let mut state = ParseState::new();
    match parse_captured_pieces(input, &mut state, 0) {
        Matched(pos, value) => { if pos == input.len() { return Ok(value) } }
        _ => { }
    }
    let (line, col) = pos_to_line(input, state.max_err_pos);
    Err(format!("Error at Line {}:{}: Expected {:?}" , line , col , state .
                expected))
}
pub fn board<'input>(input: &'input str)
 -> Result<(VecMap<(PieceType, Player)>, usize, usize), String> {
    let mut state = ParseState::new();
    match parse_board(input, &mut state, 0) {
        Matched(pos, value) => { if pos == input.len() { return Ok(value) } }
        _ => { }
    }
    let (line, col) = pos_to_line(input, state.max_err_pos);
    Err(format!("Error at Line {}:{}: Expected {:?}" , line , col , state .
                expected))
}
