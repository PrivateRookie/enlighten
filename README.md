# enlighten
用于古诗文浏览和背诵的终端APP

Power By [Cursive](https://github.com/gyscos/cursive)

感谢 [中华古诗文数据库](https://github.com/caoxingyu/chinese-gushiwen) 提供的API!

## 安装

安装 ncurses

### Archlinux

```
pacman -S ncurses
```

### Ubuntu

```
apt-get install libncursesw5-dev
```

### Fedora

```
yum install ncurses-devel
```

### macOS

```
brew install ncurses
```

安装 enlighten

```bash
cargo install --git https://github.com/PrivateRookie/enlighten.git

enlighten
```

![demo](assets/demo.gif)


## TODO

- [x] 支持所有中华古诗文数据库API(API都已对接，作者和名句页面WIP)
- [x] 添加可变mask背诵功能
- [ ] 增加历史记录功能
- [ ] 添加收藏夹功能
- [ ] 添加快捷键绑定
- [ ] 添加朗诵功能
- [ ] 添加主题设置
- [ ] 优化界面UI