use chess::{Board, BoardStatus, ChessMove, Color, MoveGen, Piece};
use std::time::{Duration, Instant};
use crate::piece_square_tables::{BISHOP_PST, EG_KING_PST, KNIGHT_PST, MG_KING_PST, PAWN_PST, QUEEN_PST, ROOK_PST};

pub fn best_move(
    has_pruning: &bool,
    board: &Board,
    depth: u32,
    maximizing: bool,
    count: &mut i64,
    time_elapsed: &mut Duration,
) -> (i32, Option<ChessMove>) {
    let result: (i32, Option<ChessMove>);
    let now = Instant::now();

    *count = 0;

    if *has_pruning {
        result = minimax_alpha_beta(
            board,
            depth,
            i32::MIN,
            i32::MAX,
            maximizing,
            count);
    } else {
        result = minimax(
            board,
            depth,
            maximizing,
            count,
        );
    };

    *time_elapsed = now.elapsed();

    result
}

pub fn evaluate_board(board: &Board) -> i32 {
    let mut result = 0;

    for square in *board.color_combined(Color::White) {
        let piece = board.piece_on(square).unwrap();
        result += piece_value(board.piece_on(square).unwrap());
        result += piece_square_value(board, piece, square, Color::White);
    }

    for square in *board.color_combined(Color::Black) {
        let piece = board.piece_on(square).unwrap();
        result -= piece_value(board.piece_on(square).unwrap());
        result -= piece_square_value(board, piece, square, Color::Black);
    }

    result
}

fn piece_value(piece: Piece) -> i32 {
    match piece {
        Piece::Pawn => 100,
        Piece::Knight => 320,
        Piece::Bishop => 330,
        Piece::Rook => 500,
        Piece::Queen => 900,
        Piece::King => 20000,
    }
}

fn piece_square_value(board: &Board, piece: Piece, square: chess::Square, color: Color) -> i32 {
    let index = square.to_index();
    match piece {
        Piece::Pawn => if color == Color::White { PAWN_PST[index] } else { PAWN_PST[mirror_index(index)] },
        Piece::Knight => if color == Color::White { KNIGHT_PST[index] } else { KNIGHT_PST[mirror_index(index)] },
        Piece::Bishop =>  if color == Color::White { BISHOP_PST[index] } else { BISHOP_PST[mirror_index(index)] },
        Piece::Rook =>  if color == Color::White { ROOK_PST[index] } else { ROOK_PST[mirror_index(index)] },
        Piece::Queen =>  if color == Color::White { QUEEN_PST[index] } else { QUEEN_PST[mirror_index(index)] },
        Piece::King =>  {

            if !check_special_endgame(board) {
                if color == Color::White { 
                    MG_KING_PST[index] 
                } else { 
                    MG_KING_PST[mirror_index(index)] 
                }
            } else {
                if color == Color::White { 
                    EG_KING_PST[index] 
                } else { 
                    EG_KING_PST[mirror_index(index)] 
                }
            }      
        },    
    }
}

fn check_special_endgame(board: &Board) -> bool {
    
    let white_queens = (board.pieces(Piece::Queen) & board.color_combined(Color::White)).popcnt();
    let black_queens = (board.pieces(Piece::Queen) & board.color_combined(Color::Black)).popcnt();
    
    let white_count = count_pieces(board, Color::White);
    let black_count = count_pieces(board, Color::Black);

    let white_condition = (white_queens == 0) || (white_queens == 1 && white_count <= 1);
    let black_condition = (black_queens == 0) || (black_queens == 1 && black_count <= 1);

    white_condition && black_condition
}

fn count_pieces(board: &Board, color: Color) -> u32 {
    let bishops = (board.pieces(Piece::Bishop) & board.color_combined(color)).popcnt();
    let knights = (board.pieces(Piece::Knight) & board.color_combined(color)).popcnt();
    let rooks = (board.pieces(Piece::Rook) & board.color_combined(color)).popcnt();
    let queens = (board.pieces(Piece::Queen) & board.color_combined(color)).popcnt();
    
    bishops + knights + rooks + queens
}

fn mirror_index(index: usize) -> usize {
    63 - index
}

fn minimax(
    board: &Board,
    depth: u32,
    maximizing: bool,
    count: &mut i64,
) -> (i32, Option<ChessMove>) {
    *count += 1;

    // ponto de parada da recursao:
    // eh necessario checar se chegou a profundidade estipulada
    // ou se board apresenta um jogo finalizado
    if depth == 0 || board.status() != BoardStatus::Ongoing {
        return (evaluate_board(board), None);
    }

    let mut best_move = None;
    let mut best_score = if maximizing { i32::MIN } else { i32::MAX };

    // itera por todos os movimentos legais no estado do tabuleiro atual
    for mv in MoveGen::new_legal(board) {
        let new_board = board.make_move_new(mv); //tabuleiro que representa um possivel movimento
        let (score, _) = minimax(&new_board, depth - 1, !maximizing, count);

        //se for a vez das brancas:
        if maximizing {
            if score > best_score {
                best_score = score;
                best_move = Some(mv);
            }
        } else {
            //se for a vez das pretas:
            if score < best_score {
                best_score = score;
                best_move = Some(mv);
            }
        }
    }

    (best_score, best_move)
}

fn minimax_alpha_beta(
    board: &Board,
    depth: u32,
    alpha: i32,
    beta: i32,
    maximizing: bool,
    count: &mut i64,
) -> (i32, Option<ChessMove>) {
    *count += 1;

    // ponto de parada da recursao:
    // eh necessario checar se chegou a profundidade estipulada
    // ou se board apresenta um jogo finalizado
    if depth == 0 || board.status() != BoardStatus::Ongoing {
        return (evaluate_board(board), None);
    }

    // representa o maior resultado encontrado naquele caminho
    // o valor inicial eh o menor possivel, pois nenhum valor foi procurado ainda
    let mut alpha = alpha;

    // representa o menor resultado encontrado naquele caminho
    // o valor inicial eh o maior possivel, pois nenhum valor foi procurado ainda
    let mut beta = beta;
    let mut best_move = None;
    let mut best_score = if maximizing { i32::MIN } else { i32::MAX };

    // itera por todos os movimentos legais no estado do tabuleiro atual
    for mv in MoveGen::new_legal(board) {
        let new_board = board.make_move_new(mv); //tabuleiro que representa um possivel movimento
        let (score, _) = minimax_alpha_beta(&new_board, depth - 1, alpha, beta, !maximizing, count);

        //se for a vez das brancas:
        if maximizing {
            if score > best_score {
                best_score = score;
                best_move = Some(mv);
            }

            //se o score for o maior encontrado ate agr:
            alpha = alpha.max(score);
        } else {
            //se for a vez das pretas:
            if score < best_score {
                best_score = score;
                best_move = Some(mv);
            }

            //se o score for o menor encontrado ate agora
            beta = beta.min(score);
        }

        //se beta for maior que alpha significa q um caminho mais favoravel ja foi garantido, ent nao precisa continuar o for loop
        if beta <= alpha {
            break;
        }
    }

    (best_score, best_move)
}
