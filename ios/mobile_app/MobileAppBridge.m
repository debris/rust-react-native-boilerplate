//
//  MobileAppBridge.m
//  mobile_app
//
//  Created by Marek Kotewicz on 18/08/2017.
//  Copyright © 2017 Facebook. All rights reserved.
//

#import <React/RCTBridgeModule.h>

@interface RCT_EXTERN_MODULE(MobileAppBridge, NSObject)

RCT_EXTERN_METHOD(sayHelloWorld:(NSString*)name resolve:(RCTPromiseResolveBlock)resolve reject:(RCTPromiseRejectBlock)reject)

@end
