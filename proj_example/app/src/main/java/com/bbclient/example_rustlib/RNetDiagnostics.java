package com.bbclient.example_rustlib;

public class RNetDiagnostics {
    static {
        System.loadLibrary("tracer_netdiagnostics");
    }

    //base
    private static native String greeting(final String pattern);

    public static native void helloasync(IRNetCallback callback);

    //NetDiagnostics
    public static native void traceroute(IRNetCallback callback);
}
