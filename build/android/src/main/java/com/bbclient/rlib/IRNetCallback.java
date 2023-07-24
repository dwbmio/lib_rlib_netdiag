package com.bbclient.rlib;

public interface IRNetCallback {
    default void perNodeCallback(int cur){};
    default void endCallback(){};
    void pingResult(String ret);
}
