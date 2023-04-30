# How

`how`命令用于查询 `linux` 命令行如何使用。资源来源于 [jaywcjlove/linux-command](https://github.com/jaywcjlove/linux-command) 项目的整理。

## 安装

cargo

```
git clone https://github.com/kingzcheung/how
cargo install how
```

## 使用

```
how ls # 查看 `ls` 命令怎么使用
```

output
```
❯ how cd
cd
===

切换用户当前工作目录。

## 概要
....
```

不知道命令名称也可以使用，比如我要找关于列表的命令：

```
how 列表
```
output:

```
❯ how 列表

您要找的是不是下面这些命令:

lsof           : 显示Linux系统当前已打开的所有文件列表 `lsof -p pid`
dpkg-divert    : Debian Linux中创建并管理一个转向列表
history        : 显示或操作历史列表。
setfacl        : 设置文件访问控制列表
lsusb          : 显示本机的USB设备列表信息
exportfs       : 管理NFS共享文件系统列表
fc             : 显示历史列表中的命令或修改指定的历史命令并执行。
ls             : 显示目录内容列表
atq            : 列出当前用户的at任务列表
ldd            : 打印程序或者库文件所依赖的共享库列表
pvscan         : 扫描系统中所有硬盘的物理卷列表
```