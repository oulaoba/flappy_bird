pub mod bird {

    use bracket_lib::prelude::BTerm;
    use bracket_lib::terminal::{to_cp437, BLACK, YELLOW_GREEN};

    /// 小鸟
    pub struct Bird {
        /// x坐标
        x: f64,
        /// y坐标
        y: f64,
        /// 下落速度
        velocity: f64,
    }

    impl Bird {
        /// 初始化小鸟
        pub fn new(game_width: u32, game_height: u32) -> Self {
            Self {
                x: (game_width / 5) as f64,
                y: (game_height / 2) as f64,
                velocity: 0.1,
            }
        }

        /// 绘制
        pub fn draw(&self, ctx: &mut BTerm) {
            ctx.set(
                self.x as u32,
                self.y as u32,
                YELLOW_GREEN,
                BLACK,
                to_cp437('&'),
            );
        }

        /// 获取当前的 x 坐标
        pub fn bird_x(&self) -> f64 {
            self.x
        }

        /// 获取当前的 y 坐标
        pub fn bird_y(&self) -> f64 {
            self.y
        }

        /// 自由下落
        pub fn gravity_and_move(&mut self) {
            self.velocity += 0.1;
            self.y += self.get_velocity();
        }

        /// 点击空格向上
        pub fn flap(&mut self) {
            self.y -= 2.0;
            self.velocity = 0.1;
        }

        /// 是否超出边界
        pub fn bird_out(&self, max_y: u32) -> bool {
            self.y as u32 > max_y || self.y < 0.0
        }

        /// 获取当前下落的速度，限制到了2.0
        fn get_velocity(&mut self) -> f64 {
            if self.velocity > 2.0 {
                self.velocity = 2.0
            }
            self.velocity
        }
    }
}
