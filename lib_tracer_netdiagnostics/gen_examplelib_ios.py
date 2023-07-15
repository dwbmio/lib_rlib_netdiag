# encoding:utf8
import os
import sys
import shutil

TARGET_LIB_NAME = "libtracer_netdiagnostics.so"

def copy_libstatic(target_os: str = "android", target_arch="aarch64-linux-android"):
    print("copy lib...")
    cur_dir = os.path.curdir
    f_lib = os.path.join(cur_dir, "target", target_arch,
                         "release", TARGET_LIB_NAME)

    t_lib_par = "arm64-v8a" if target_arch == "aarch64-linux-android" else "armeabi-v7a"
    t_lib = os.path.join(cur_dir, os.pardir, "example",
                         "app", "libs", t_lib_par, TARGET_LIB_NAME)
    shutil.copyfile(f_lib, t_lib)
    print("done!")


def android_build(arch: str = "aarch64-linux-android"):
    print("cargo build...")
    cmd_str = "cargo build --target {arch} --release".format(arch=arch)
    print("run cmd->", cmd_str)
    b_ret = os.system(cmd_str)
    if b_ret != 0:
        sys.exit(1)
    print("cargo build suc!")


if __name__ == "__main__":
    android_build()
    copy_libstatic()
