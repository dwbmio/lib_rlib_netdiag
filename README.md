
## Overview

主要依赖[surge-ping](https://crates.io/crates/surge-ping)

如何使用rust开发android应用
[参照教程](https://juejin.cn/post/7170696817682694152)


## Require 

* netdiag v0.3.0
[crate.io](https://crates.io/crates/netdiag)

支持系统基本信息查询


## Build 

---
* iOS 

--- 

* Android 
>cargo build --target aarch64-linux-android --release \
>cargo build --target armv7-linux-androideabi --release \
>cargo build --target i686-linux-android --release



## Usage
Step1. 

Copy the build result to xcode\gradle project.

And add the dependenceise.


---

**IMPORTANT**\
移动平台使用前问题排查！

* Android\
1.AndroidMenifest.xml里面是否有INTENET权限 \
2.申请的端口是否是Android操作系统允许(>33k) \
3.ICMP协议在Android上的完整实现

* iOS\
1.Info.plist网络权限 


Step2.
