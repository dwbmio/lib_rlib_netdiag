# encoding:utf8
import os
import sys
import shutil

TARGET_LIB_NAME = "libtracer_netdiagnostics.so"

def gen_and_copy2_demo(target_os: str = "android", target_arch="aarch64-linux-android"):
    print("copy lib...")
    cur_dir = os.path.curdir
    f_lib = os.path.join(cur_dir, "target", target_arch,
                         "release", TARGET_LIB_NAME)
    def _android():
        t_lib_par = "arm64-v8a" if target_arch == "aarch64-linux-android" else "armeabi-v7a"
        t_lib = os.path.join(cur_dir, os.pardir, "example", "android",
                         "app", "libs", t_lib_par, TARGET_LIB_NAME)
        return t_lib


    def _ios():
        t_lib_par = "aarch64-apple-ios"
        t_lib = os.path.join(cur_dir, os.pardir, "example", "ios")
        return t_lib

    tar_path = dict(
        android = _android, 
        ios = _ios
    )
    shutil.copyfile(f_lib, tar_path[target_os]())
    print("done!")


def rustlib_gen(arch: str = "aarch64-linux-android"):
    print("cargo build...")
    cmd_str = "cargo build --target {arch} --release".format(arch=arch)
    print("run cmd->", cmd_str)
    b_ret = os.system(cmd_str)
    if b_ret != 0:
        sys.exit(1)
    print("cargo build suc!")


if __name__ == "__main__":
    # android default 
    if len(sys.argv) == 1:
        sys.argv.append("android")

    if sys.argv[1] == "android":
        rustlib_gen()
        gen_and_copy2_demo()
    elif sys.argv[1] == "ios":
        rustlib_gen("aarch64-apple-ios")
        gen_and_copy2_demo("ios")
