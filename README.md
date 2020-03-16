# mikack-favicon

图标生成工具，自动构建 [mikack](https://github.com/Hentioe/mikack) 项目支持的所有平台图标。

## 生成图标

每运行一次自动下载最新的原始图标。当前不支持对图标进行二次统一处理，故没有多余的参数。

### 安装项目

```bash
cargo install --path .
```

### 运行程序

```bash
mikack-favicon
```

查看输出结果，确认是否存在网络或其它情况导致没有完整下载所有图标。可能会有 ignored，因为不是每一个平台都提供图标。

### 输出目录

```bash
dist
├── 18h.animezilla.com.ico
├── 8comic.se.ico
├── 9hentai.com.png
├── comic.ikkdm.com.ico
├── c-upp.com.ico
├── e-hentai.org.ico
├── loveheaven.net.ico
├── manganelo.com.ico
├── manhua.dmzj.com.ico
├── nhentai.net.ico
├── www.177mh.net.ico
├── www.177pic.info.ico
├── www.2animx.com.ico
├── www.bidongmh.com.ico
├── www.bnmanhua.com.ico
├── www.cartoonmad.com.ico
├── www.comico.com.tw.ico
├── www.dm5.com.ico
├── www.gufengmh8.com.ico
├── www.hhimm.com.ico
├── www.kuaikanmanhua.com.ico
├── www.luscious.net.ico
├── www.mangabz.com.ico
├── www.manhuadb.com.ico
├── www.manhuadui.com.ico
├── www.manhuagui.com.ico
├── www.onemanhua.com.png
├── www.pufei8.com.ico
├── www.qimiaomh.com.ico
├── www.tohomh123.com.ico
└── www.wnacg.org.ico

0 directories, 31 files
```

## 使用图标

因为网页的 favicon 尺寸普遍太小，经过常见的图像缩放算法显示在 GUI 上会出现异常模糊的反效果。为了避免这种情况，mikack 官方涉及到的所有项目都使用紧邻插值算法放大图像。