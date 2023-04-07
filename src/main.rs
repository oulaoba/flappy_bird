use bracket_lib::prelude::{main_loop, BError, BTermBuilder};

pub mod bird;
pub mod game;
pub mod obstacles;

fn main() -> BError {
    // 创建窗口
    let ctx = BTermBuilder::simple80x50()
        .with_title("Flappy Bird")
        .build()
        .unwrap();

    // 创建游戏内容
    let game = game::game::State::new();

    // 初始化游戏 loop
    main_loop(ctx, game)
}
