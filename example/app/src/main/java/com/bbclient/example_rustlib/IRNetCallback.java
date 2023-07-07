package com.bbclient.example_rustlib;

public interface IRNetCallback {
    void perNodeCallback(int cur);
    void endCallback();
    void pingsResult(String ret);
}
