# phigbot

[Tips <= 1.5.0](https://www.bilibili.com/read/cv12614938), [all Tips](https://www.bilibili.com/read/cv15210488/)

暂时是个可以在telegram上输出Phigros的Tips中的tip的telegram bot（逃

## 用法

### inline方式

啥都没有：随机返回一条tip
缓存时间 0

*：返回到mod.rs，也就是全部tips的链接
缓存时间 u32::MAX

任意输入：返回大小写不敏感的搜索结果。
缓存时间：10分钟

### 命令方式

暂无