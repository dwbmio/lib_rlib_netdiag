# encoding:utf8
import os
import sys
import shutil



def gen_and_copy2_demo(target_os: str = "android", target_arch="aarch64-linux-android"):
    print("copy lib...")
    cur_dir = os.path.curdir
    
    def _android():
        TARGET_LIB_NAME = "libtracer_netdiagnostics.so"
        t_lib_par = "arm64-v8a" if target_arch == "aarch64-linux-android" else "armeabi-v7a"
        f_lib = os.path.join(cur_dir, "target", target_arch,
                         "release", TARGET_LIB_NAME)
        t_lib = os.path.join(cur_dir, os.pardir, "build", "android", 
                         "app", "libs", t_lib_par, TARGET_LIB_NAME)
        return (f_lib, t_lib)


    def _ios():
        TARGET_LIB_NAME = "libtracer_netdiagnostics.a"
        t_lib_par = "aarch64-apple-ios"
        f_lib = os.path.join(cur_dir, "target", t_lib_par,
                         "release", TARGET_LIB_NAME)
        t_lib = os.path.join(cur_dir, os.pardir, "build", "ios", "net-diagnosis", TARGET_LIB_NAME)
        return (f_lib, t_lib)

    tar_path = dict(
        android = _android, 
        ios = _ios
    )
    (f_lib, t_lib) = tar_path[target_os]()
    shutil.copyfile(f_lib, t_lib)
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
