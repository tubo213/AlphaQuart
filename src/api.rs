use actix_web::{web, HttpResponse, Responder};
use alpha_quart::game::{Game, Player};
use alpha_quart::policies::{Policy, RandomPolicy};
use std::sync::Mutex;

// レスポンスの形式を定義する構造体
#[derive(serde::Serialize)]
struct GameResponse {
    game: Game,
}

// プレイの要求を受け取る構造体
#[derive(serde::Deserialize)]
struct PlayTurnRequest {
    row: usize,
    col: usize,
    piece_index: usize,
}

// 新しいゲームを開始するエンドポイント
pub async fn new_game() -> impl Responder {
    // 新しいゲームを初期化する
    let game = Game::new();
    // プレイヤー1とプレイヤー2にランダム戦略を設定
    let player1 = RandomPolicy::new();
    let player2 = RandomPolicy::new();

    // プレイヤー1がHuman（ユーザー入力）で、プレイヤー2がComputer（ランダム戦略）と仮定
    let game_data = web::Data::new(Mutex::new((game, player1, player2)));

    // 初期ゲーム状態をJSONで返す
    HttpResponse::Ok().json(GameResponse { game: game_data.lock().unwrap().0.clone() })
}

// プレイのターンを処理するエンドポイント
pub async fn play_turn(
    game_data: web::Data<Mutex<(Game, RandomPolicy, RandomPolicy)>>,
    play_turn_request: web::Json<PlayTurnRequest>,
) -> impl Responder {
    let mut game_data = game_data.lock().unwrap();
    let (game, player1, player2) = &mut *game_data;

    let PlayTurnRequest {
        row,
        col,
        piece_index,
    } = play_turn_request.into_inner();

    // Humanのターンを処理
    if game.current_player == Player::Player1 {
        // プレイを実行し、エラーがあれば返す
        if let Err(error) = game.play_turn(row, col, piece_index) {
            return HttpResponse::BadRequest().body(error);
        }

        // プレイヤーを切り替え
        game.switch_player();
    }

    // Computerのターンを処理（Humanの後）
    if game.current_player == Player::Player2 {
        let action = player2.action(game);
        if let Err(error) = game.play_turn(action.row, action.col, action.piece_index) {
            return HttpResponse::BadRequest().body(error);
        }

        // プレイヤーを切り替え
        game.switch_player();
    }

    // ゲームが終了しているか確認
    if game.is_game_over() {
        HttpResponse::Ok().json(GameResponse { game: game.clone() })
    } else {
        HttpResponse::Ok().json(GameResponse { game: game.clone() })
    }
}
