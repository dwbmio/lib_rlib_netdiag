package com.bbclient.example_rustlib;

import android.os.Bundle;

import androidx.appcompat.app.AppCompatActivity;

import android.util.Log;
import android.view.View;

import com.bbclient.example_rustlib.databinding.ActivityMainBinding;

public class MainActivity extends AppCompatActivity {
    private ActivityMainBinding binding;

    public static String LOG_TAG = "[MainActivity]";

    public void asyncCallback(int progress) {
        System.out.println("asyncCallback: thread id = " + Thread.currentThread().getId() + ", progress = " + progress + "%");
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);

        binding = ActivityMainBinding.inflate(getLayoutInflater());
        setContentView(binding.getRoot());


        var btn = binding.getRoot().getChildAt(1);

        var callback = new IRNetCallback() {

            @Override
            public void perNodeCallback(int progress) {
                System.out.println("asyncCallback: thread id = " + Thread.currentThread().getId() + ", progress = " + progress + "%");
            }

            @Override
            public void endCallback() {

            }
        };
        btn.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View v) {
                Log.i(LOG_TAG, "call init...");
                RNetDiagnostics.init();
                Log.i(LOG_TAG, "call greeting...");
                var ret = RNetDiagnostics.greeting("from mainactivity");
                Log.i(LOG_TAG, ret);
                Log.i(LOG_TAG, "call traceroute...");
                RNetDiagnostics.traceroute(callback);
            }
        });
    }
}