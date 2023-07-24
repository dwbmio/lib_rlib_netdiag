package com.bbclient.example_rustlib;

import android.os.Bundle;
import android.util.Log;
import android.view.View;

import androidx.appcompat.app.AppCompatActivity;

import com.bbclient.example_rustlib.databinding.ActivityMainBinding;
import com.bbclient.rlib.IRNetCallback;
import com.bbclient.rlib.NetDiagnostics;

public class MainActivity extends AppCompatActivity {
    private ActivityMainBinding binding;

    public static String LOG_TAG = "[MainActivity]";

    public void asyncCallback(int progress) {
        System.out.println("asyncCallback: thread id = " + Thread.currentThread().getId() + ", progress = " + progress + "%");
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        Log.i(LOG_TAG, "call init...");
        NetDiagnostics.load();

        binding = ActivityMainBinding.inflate(getLayoutInflater());
        setContentView(binding.getRoot());


        var btn = binding.getRoot().getChildAt(1);

        var callback = new IRNetCallback() {
            @Override
            public void pingResult(String ret) {
                Log.i(LOG_TAG, String.format("-->>>ping result is %s", ret));
            }
        };
        btn.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View v) {
                NetDiagnostics.ping("www.baidu.com", callback);
            }
        });
    }
}