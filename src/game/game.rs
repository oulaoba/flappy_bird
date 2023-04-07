pub mod game {
    use crate::bird::bird::bird;
    use crate::obstacles::obstacle::obstacle;
    use bracket_lib::{
        prelude::{BTerm, GameState},
        terminal::{VirtualKeyCode, LIGHT_BLUE},
    };
    use std::collections::LinkedList;

    /// 游戏窗口的宽
    const GAME_WIDTH: u32 = 80;
    /// 游戏窗口的高
    const GAME_HEIGHT: u32 = 50;
    /// 游戏周期
    const PERIOD: f32 = 120.0;

    /// 游戏模式
    enum GameMode {
        /// 游戏中
        Playing,
        /// 游戏结束
        End,
        /// 游戏暂停
        Paused,
        /// 菜单页面
        Menu,
    }

    /// 游戏 State
    pub struct State {
        /// 当前模式
        mode: GameMode,
        /// 得分
        score: u32,
        /// 小鸟
        bird: bird::Bird,
        /// 等待时间
        waite_time: f32,
        /// 障碍物列表，因为需要从头尾两处进行操作，所以选择使用双向链表
        obstacle_list: LinkedList<obstacle::OBstacle>,
    }

    /// bracket_lib::prelude::State 的实现
    impl GameState for State {
        fn tick(&mut self, ctx: &mut BTerm) {
            match self.mode {
                GameMode::Playing => self.paly_control(ctx),
                GameMode::End => self.end_control(ctx),
                GameMode::Menu => self.menu_control(ctx),
                GameMode::Paused => self.paused_control(ctx),
            }
        }
    }

    impl State {
        /// 游戏状态初始化
        pub fn new() -> Self {
            let mut obstacle_list = LinkedList::new();
            obstacle_list.push_back(obstacle::OBstacle::new(0, 35, GAME_HEIGHT));
            obstacle_list.push_back(obstacle::OBstacle::new(0, 50, GAME_HEIGHT));
            obstacle_list.push_back(obstacle::OBstacle::new(0, 65, GAME_HEIGHT));
            obstacle_list.push_back(obstacle::OBstacle::new(0, 80, GAME_HEIGHT));
            Self {
                mode: GameMode::Menu,
                score: 0,
                bird: bird::Bird::new(GAME_WIDTH, GAME_HEIGHT),
                waite_time: 0.0,
                obstacle_list,
            }
        }

        /// 游戏中的控制方法
        fn paly_control(&mut self, ctx: &mut BTerm) {
            ctx.cls();
            ctx.set_bg(GAME_WIDTH, GAME_HEIGHT, LIGHT_BLUE);

            // tick 函数的调用周期 ctx.frame_time_ms =33
            self.waite_time += ctx.frame_time_ms;

            // 达到等待时间了，需要执行游戏内容
            if self.waite_time >= PERIOD {
                self.bird.gravity_and_move();
                self.move_obstacles();
                self.pass_obstacles();
                self.waite_time = 0.0;
            }

            if let Some(key) = ctx.key {
                match key {
                    VirtualKeyCode::S => self.mode = GameMode::Playing,
                    VirtualKeyCode::P => self.mode = GameMode::Paused,
                    VirtualKeyCode::Space => {
                        self.bird.flap();
                        self.bird.draw(ctx)
                    }
                    VirtualKeyCode::Q | VirtualKeyCode::Escape => ctx.quit(),
                    _ => (),
                }
            }
            self.bird.draw(ctx);

            for obt in &self.obstacle_list {
                obt.draw(GAME_HEIGHT, ctx)
            }
            ctx.print(0, 0, format!("Score :{}", self.score));
            ctx.print(0, 1, "Space to flap");

            if self.bird.bird_out(GAME_HEIGHT) {
                self.mode = GameMode::End
            }
        }

        /// 通过障碍物检测，并记分
        fn pass_obstacles(&mut self) {
            let first_obt = self.obstacle_list.front().unwrap();
            if first_obt.obstacle_x() <= self.bird.bird_x() as u32 {
                let half_sie = first_obt.obstacle_size() / 2;
                let first_top = first_obt.obstacle_gap_y() - half_sie;
                let first_bottom = first_obt.obstacle_gap_y() + half_sie;
                let dragon_y = self.bird.bird_y() as u32;

                if dragon_y > first_top && dragon_y < first_bottom {
                    self.score += 1;
                } else {
                    self.mode = GameMode::End;
                }

                self.obstacle_list.pop_front();
                self.obstacle_list
                    .push_back(obstacle::OBstacle::new(self.score, 80, GAME_HEIGHT));
            }
        }

        /// 移动障碍物
        fn move_obstacles(&mut self) {
            for obt in &mut self.obstacle_list {
                obt.move_forward(1);
            }
        }

        /// 游戏结束时的控制方法
        fn end_control(&mut self, ctx: &mut BTerm) {
            ctx.cls();
            ctx.print_centered(6, format!("You are DEAD ! with {} score. ", self.score));
            ctx.print_centered(8, " (R)  Restart game");
            ctx.print_centered(9, " (Q/Esc) Exit ");

            if let Some(key) = ctx.key {
                match key {
                    VirtualKeyCode::R => {
                        self.reset();
                        self.mode = GameMode::Playing;
                    }
                    VirtualKeyCode::Q | VirtualKeyCode::Escape => ctx.quitting = true,
                    _ => (),
                }
            }
        }

        /// 菜单状态的控制方法
        fn menu_control(&mut self, ctx: &mut BTerm) {
            ctx.cls();
            ctx.print_centered(6, "Welcome to flappy dragon ");
            ctx.print_centered(8, " (S) Start game");
            ctx.print_centered(9, " (P) Paused game");
            ctx.print_centered(10, " (Q/Esc) Exit ");

            if let Some(key) = ctx.key {
                match key {
                    VirtualKeyCode::S => self.mode = GameMode::Playing,
                    VirtualKeyCode::Q | VirtualKeyCode::Escape => ctx.quit(),
                    _ => (),
                }
            }
        }

        /// 游戏暂停时的控制方法
        fn paused_control(&mut self, ctx: &mut BTerm) {
            ctx.print_centered(0, format!("Score :{}", self.score));
            ctx.print_centered(7, "Game Paused, (P) to return game!");
            self.bird.draw(ctx);
            if let Some(key) = ctx.key {
                match key {
                    VirtualKeyCode::P => self.mode = GameMode::Playing,
                    VirtualKeyCode::Q | VirtualKeyCode::Escape => ctx.quit(),
                    _ => (),
                }
            }
        }

        /// 重置游戏内容的方法
        fn reset(&mut self) {
            self.mode = GameMode::Playing;
            self.waite_time = 0.0;
            self.bird = bird::Bird::new(GAME_WIDTH, GAME_HEIGHT);
            self.score = 0;
            let mut obstacle_list = LinkedList::new();
            obstacle_list.push_back(obstacle::OBstacle::new(0, 35, GAME_HEIGHT));
            obstacle_list.push_back(obstacle::OBstacle::new(0, 50, GAME_HEIGHT));
            obstacle_list.push_back(obstacle::OBstacle::new(0, 65, GAME_HEIGHT));
            obstacle_list.push_back(obstacle::OBstacle::new(0, 80, GAME_HEIGHT));
            self.obstacle_list = obstacle_list;
        }
    }
}
