# Common Virtual Machine Binary Format
> version 0.0.1 alpha

## File Format
- Magic Number
- CheckSum
  - Method
  - Data
- VM Specifier
- VM Info
- Compression
  - Method
  - Password
- Library Version
  - Major
  - Minor
  - Patch
- Library EXT Info
- Access(Such as Package or Module etc)
- Dependencies(Optional)
- Constant Pool
  - Strings
  - Layout Section
  - Type Section
- Info (Optional)
- Static(Global) Section
- Executable Section

## Binary Kindes
CVM 暂定文件类型

0. 元信息: 仅包含元信息的文件
1. 库文件: 有 EntryPoint 时视为可执行文件
2. 单一可执行文件: 对应 Jar 包
3. 分离常量池的库文件: 仅包含元信息和可执行代码的库文件

---
## Magic Number
- [x] complete


CVMB in ASCII
```
0x43 0x56 0x4D 0x42 0x0A
```

## Check Sum
- [x] complete

方法:
- CRC
- MD5
- SHA

Data

## VM Specifier
- [x] complete

for example
```
LemonVM The Best VM ever existed in the fucking universe
```
see String Type

## VM Info
- [x] complete

运行这个库的VM的信息

for example
```
version: 1.0.0
extensions: SIMD SoftFloat JIT
platform: POSIX
thread: green-thread
```
len + custom info

## Compression
- [x] complete

就是把字节码剩下的所有部分全部压缩

## Library Version
- [x] complete

see [semantic version](https://semver.org/)

## Library EXT Info
- [x] complete


len + custom info

for example:
```
extensions: aaa, bbb, ccc, ddd
global env: yyy, zzz
```


## Access
- [x] complete

len + custom info

for example
```
package: org.lemonvm.yyy.zzz
module: ...
```

## Dependencies
- [x] complete

len + custom info

for example:
```
org.lemonscript.float
org.aaa.bbb
```

## Constant Pool
常量池分为三个部分
1. string data
2. layout info
3. type info
4. constants

### String Data
- [x] complete
就是一堆bytes
```
0x00 0x00 0x00 0x00 0x00 0x00 ...
```

### Layout Info
- [ ] complete

有一个对应的UUID
描述了数据类型的layout
然后就是每个项对应的offset
比如有
```
struct {
  i32 xxx
  i64 yyy
  u8 zzz
}
```
该格式的人类可读格式会是如下
```
size 16 xxx 0 yyy 4 zzz 12
```
比方说困难一点的格式
```
struct {
  union {
    i32 a
    u8 b
  }
  i32 c
}
```
```
size 8 a 0 b 0 c 4
```
数组类型
```
struct {
  i32[114514] a
}
```
```
size 114514*4 a 0
```
### Type Info
- [ ] complete

包含了类型信息
有一个UUID还有一组跟Layout对应的UUID
这个应该会更方便做ADT类型
名字存储在 `Constants` 中

### Constants
- [ ] complete

有对应`Layout`的UUID还有一个`Strings`的Offset

TODO: 数据类型
TODO: 指针类型

## Info
- [ ] complete


应该存储关于单个函数或全局变量的相关信息如访问控制， 平台相关编译信息,调试信息等.

## Static Global Section
- [ ] complete


如果你把一个文件当一个类似`Java`的类来使用的话这里应该存放的是每一个`field`的信息, 如果是类似`C`系列的语言就可以放置一些`static`相关的信息, 反正就是这个语义

## Executable Section
- [ ] complete

TODO

---

## 分发打包
- 单一可执行文件,但没有虚拟机
- <需要lld/ld> 可以注册虚拟机静态链接库之后打包成该平台下单一可执行文件
  - ELF
  - COFF
  - MACH-O
