//
//  lib_netdiagnosisTests.m
//  lib_netdiagnosisTests
//
//  Created by homework on 2023/7/24.
//

#import <XCTest/XCTest.h>
#import <lib_netdiagnosis/net_diagnosis.h>

@interface lib_netdiagnosisTests : XCTestCase

@end

@implementation lib_netdiagnosisTests

- (void)setUp {
    // Put setup code here. This method is called before the invocation of each test method in the class.
}

- (void)tearDown {
    // Put teardown code here. This method is called after the invocation of each test method in the class.
}

- (void)testExample {
    // This is an example of a functional test case.
    // Use XCTAssert and related functions to verify your tests produce the correct results.
    r_netdiag_init();
    r_netdiag_ping("www.baidu.com");
}

- (void)testPerformanceExample {
    // This is an example of a performance test case.
    [self measureBlock:^{
        // Put the code you want to measure the time of here.
    }];
}

@end
