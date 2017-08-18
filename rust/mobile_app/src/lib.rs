extern crate libc;

mod string;

use string::StringPtr;

// string ffi

#[no_mangle]
pub unsafe extern fn rust_string_ptr(s: *mut String) -> *mut StringPtr {
  Box::into_raw(Box::new(StringPtr::from(&**s)))
}

#[no_mangle]
pub unsafe extern fn rust_string_destroy(s: *mut String) {
  let _ = Box::from_raw(s);
}

#[no_mangle]
pub unsafe extern fn rust_string_ptr_destroy(s: *mut StringPtr) {
  let _ = Box::from_raw(s);
}

#[no_mangle]
pub unsafe extern fn hello_world(name: *mut StringPtr) -> *mut String {
  let name = (*name).as_str();
  let response = format!("Hello {}!", name);
  Box::into_raw(Box::new(response))
}

#[cfg(feature = "jni")]
#[allow(non_snake_case)]
pub mod android {
  extern crate jni;

  use self::jni::JNIEnv;
  use self::jni::objects::{JClass, JString};
  use self::jni::sys::jstring;

  #[no_mangle]
  pub unsafe extern fn Java_com_mobile_1app_MobileAppBridge_helloWorld(
    env: JNIEnv, _: JClass, name: JString
  ) -> jstring {
    let name: String = env.get_string(name).unwrap().into();
    let response = format!("Hello {}!", name);
    env.new_string(response).unwrap().into_inner()
  }
}

