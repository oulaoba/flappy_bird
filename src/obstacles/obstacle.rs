pub mod obstacle {
    use bracket_lib::{
        prelude::BTerm,
        random::RandomNumberGenerator,
        terminal::{to_cp437, BLACK, RED},
    };

    /// 障碍物
    pub struct OBstacle {
        /// 障碍物的横坐标
        x: u32,
        /// 缝隙高度
        size: u32,
        /// 障碍物的中心点
        gap_y: u32,
    }

    impl OBstacle {
        /// 障碍物初始化
        ///
        /// *score 当前得分
        ///
        /// *start_x 初始x坐标
        pub fn new(score: u32, start_x: u32, screen_height: u32) -> Self {
            let mut random = RandomNumberGenerator::new();
            let size = u32::max(2, 20 - score / 3);
            let half_size = size / 2 + 1;
            Self {
                x: start_x,
                size,
                gap_y: random.range(half_size, screen_height - half_size),
            }
        }

        /// 向前移动
        ///
        /// *distance 向前移动的距离
        pub fn move_forward(&mut self, distance: u32) {
            self.x -= distance
        }

        pub fn obstacle_x(&self) -> u32 {
            self.x
        }

        pub fn obstacle_size(&self) -> u32 {
            self.size
        }

        pub fn obstacle_gap_y(&self) -> u32 {
            self.gap_y
        }

        /// 绘制
        pub fn draw(&self, screen_height: u32, ctx: &mut BTerm) {
            let screen_x = self.x;
            let half_size = self.size / 2;

            for y in 0..self.gap_y - half_size {
                ctx.set(screen_x, y, RED, BLACK, to_cp437('#'));
            }

            for y in self.gap_y + half_size..screen_height {
                ctx.set(screen_x, y, RED, BLACK, to_cp437('#'));
            }
        }
    }
}
