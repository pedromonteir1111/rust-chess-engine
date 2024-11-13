use chess::{Color, Board, Piece, BoardStatus, MoveGen, ChessMove};

fn evaluate_board(board: &Board) -> i32 {
    let mut result = 0;

    for square in *board.color_combined(Color::White){
        result += piece_value(board.piece_on(square).unwrap());
    }

    for square in *board.color_combined(Color::Black){
        result -= piece_value(board.piece_on(square).unwrap());
    }

    result
}

fn piece_value(piece: Piece) -> i32 {
    match piece {
        Piece::Pawn => 1,
        Piece::Knight | Piece::Bishop => 3,
        Piece::Rook => 5,
        Piece::Queen => 9,
        Piece::King => 0,
    }
}

pub fn minimax(board: &Board, depth: u32, alpha: i32, beta: i32, maximizing: bool) -> (i32, Option<ChessMove>) {
    
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
        let (score, _) = minimax(&new_board, depth - 1, alpha, beta, !maximizing);

        //se for a vez das brancas:
        if maximizing {
            if score > best_score {
                best_score = score;
                best_move = Some(mv);
            }

            //se o score for o maior encontrado ate agr:
            alpha = alpha.max(score);

        } else { //se for a vez das pretas:
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