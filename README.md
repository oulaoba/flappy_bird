# Rust 实现的 flappy bird

## 简介
一个非常简单的小项目。

看到了[杨旭](https://www.bilibili.com/video/BV1vM411J74S)大佬的教学视频，自己跟着实现了一下，完善了一下游戏逻辑。

通过空格键进行控制。

游戏中可按 P 键 暂停/恢复 游戏

## 代码结构
```
·
├── Cargo.lock
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├──bird/
│   │  ├── bird.rs
│   │  └── mod.rs
│   ├──game/
│   │   ├── game.rs
│   │   └── mod.rs
│   └──obstacles/
│       ├──obstacle.rs
│       └── mod.rs
```
- game.rs 负责游戏的逻辑、控制、等内容。
- snake.rs 负责小鸟本身的实现。
- obstacle.rs 负责障碍物的实现。

## about me 
目前失业，在家学习 rust 。

我的 [bilibili](https://space.bilibili.com/259260787),我的 [博客园](https://www.cnblogs.com/SantiagoZhang/)。
