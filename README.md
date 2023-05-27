
## Overview

<mark>项目暂停</mark>

[netdiag](https://crates.io/crates/netdiag)

暂时在android arm64上编译能顺利通过，但是运行时会提示

>permission denied(os error 1)

一通查看：
1.~~AndroidMenifest.xml里面是否有INTENET权限~~
2.~~申请的端口是否是Android操作系统允许(>33k)~~
3.ICMP协议在Android上的完整实现

第三天跟进，发现RawSocket创建的时候，使用的socket2 rust lib 版本还是太低了
不想改造的前提下，直接用现在还有点太早
:3等**netdiag**库再迭代迭代再说




## Require 

* netdiag v0.3.0
[crate.io](https://crates.io/crates/netdiag)

支持系统基本信息查询


## Snippet 

如何使用rust开发android应用


