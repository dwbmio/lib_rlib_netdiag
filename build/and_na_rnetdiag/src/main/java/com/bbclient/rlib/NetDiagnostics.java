package com.bbclient.rlib;

public class NetDiagnostics {

    public static void load() {
        System.loadLibrary("tracer_netdiagnostics");
        init();
    }
    /**
     * ===============FFI=================
     * */
    //Params test
    public static native String greeting(final String pattern);
    //Init android_logger
    public static native void init();
    //Ping host
    public static native void ping(final String host,  IRNetCallback callback);
    /**
     * ===============FFI=================
     * */
}
