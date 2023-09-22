# encoding:utf8
import os
import sys
import shutil


def gen_and_copy2_demo(target_os: str = "android", target_arch="aarch64-linux-android"):
    cur_dir = os.path.curdir


    def _get_output_path(armv, TARGET_LIB_NAME):
        return os.path.join(cur_dir, os.pardir, "build", "android",
                                 "app", "libs", armv, TARGET_LIB_NAME)

    def _android():
        TARGET_LIB_NAME = "libtracer_netdiagnostics.so"
        BUILD_TAR = [
            ("arm64-v8a", "aarch64-linux-android"),
            ("armeabi-v7a", "armv7-linux-androideabi"),
        ]
        ret = []
        for (armv, target_name) in BUILD_TAR:
            cmd_str = "cargo build --target {arch} --release".format(arch=target_name)
            print("run cmd->", cmd_str)
            b_ret = os.system(cmd_str)
            if b_ret != 0:
                sys.exit(1)
            f_lib = os.path.join(cur_dir, "target", target_arch,
                                 "release", TARGET_LIB_NAME)
            t_lib = _get_output_path(armv, TARGET_LIB_NAME)
            ret.append((f_lib, t_lib))
        return ret

    def _ios():
        TARGET_LIB_NAME = "libtracer_netdiagnostics.a"
        t_lib_par = "aarch64-apple-ios"
        f_lib = os.path.join(cur_dir, "target", t_lib_par,
                             "release", TARGET_LIB_NAME)
        t_lib = os.path.join(cur_dir, os.pardir, "build",
                             "ios", "net-diagnosis", TARGET_LIB_NAME)
        return (f_lib, t_lib)

    tar_path = dict(
        android=_android,
        ios=_ios
    )
    output_list = tar_path[target_os]()
    for out in output_list:
        out_dir = os.path.relpath(os.path.join(out[1], os.pardir))
        if os.path.isdir(out_dir):
            shutil.rmtree(out_dir)
        os.makedirs(out_dir)
        shutil.copyfile(out[0], out[1])
        print("output:%s"%out[1])
    print("done!")


def rustlib_gen(arch: str = "aarch64-linux-android"):
    print("cargo build...")
    cmd_str = "cargo build --target {arch} --verbose --release".format(arch=arch)
    print("run cmd->", cmd_str)
    b_ret = os.system(cmd_str)
    if b_ret != 0:
        sys.exit(1)
    print("cargo build suc!")


if __name__ == "__main__":
    # android default
    if len(sys.argv) == 1:
        sys.argv.append("android")

    plat = sys.argv[1]
    
    if plat == "android":
        gen_and_copy2_demo()
    elif plat == "ios":
        gen_and_copy2_demo("ios")
