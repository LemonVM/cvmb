# Common Virtual Machine Binary Format
> version 0.0.1 alpha

## File Format
- Magic Number
- CheckSum (Optional)
  - Method
  - Data
- Compression (Optional)
  - Method
  - Password
- VM Specifier
- VM Version
- Library Version
  - Major
  - Minor
  - Patch
- Access(Such as Package or Module etc)
- Dependencies(Optional)
- Constant Data Section
- Constant Pool
- Layout Section
- Type Section
- Debug Info (Optional)
  - Line Number
- Static(Global) Section
- Executable Section

## Binary Types
CVM 暂定文件类型

0. 元信息: 仅包含元信息的文件
1. 库文件: 有 EntryPoint 时视为可执行文件
2. 单一可执行文件: 对应 Jar 包
3. 分离常量池的库文件: 仅包含元信息和可执行代码的库文件

##