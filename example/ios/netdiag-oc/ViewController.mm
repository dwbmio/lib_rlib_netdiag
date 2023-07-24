//
//  ViewController.m
//  netdiag-oc
//
//  Created by homework on 2023/7/19.
//

#import "ViewController.h"
#import <UIKit/UIKit.h>
#import <lib_netdiagnosis/net_diagnosis.h>

@interface ViewController ()

@end

@implementation ViewController

- (void)viewDidLoad {
    r_netdiag_init();
    [super viewDidLoad];
    // Do any additional setup after loading the view.
    UIButton *button1 = [UIButton buttonWithType:UIButtonTypeRoundedRect];
    button1.frame = CGRectMake(0,200, 280, 20);
    //背景颜色
    button1.backgroundColor = [UIColor redColor];
    [button1 setTitle:@"点击1" forState:UIControlStateNormal];
    [button1 addTarget:self action:@selector(buttonClick:) forControlEvents:UIControlEventTouchUpInside];
    [self.view addSubview:button1];
    
}

//添加一个方法 来相应按钮的点击时间
- (void)buttonClick:(UIButton*)button{
    
    const char* ret = r_netdiag_ping("www.baidu.com");
    printf("%s netdiag ping result:", ret);
//    r_netdiag_init();
    //父视图通过tag值获取子视图的指针对象
    /*
     子视图可以设置一个tag值，然后添加到父视图上，父视图就可以通过这个tag值拿到子视图的指针。
     tag值也可以保存一些用户的信息。
     */
    UILabel* label = (UILabel*)[self.view.window viewWithTag:100];
    [self.view addSubview:label];
    label.text = @"我被修改了";
}


@end
