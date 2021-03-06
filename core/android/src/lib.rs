/**
 * Copyright (c) Visly Inc.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */
use core;

use json::JsonValue;
use std::any::Any;
use std::f32;
use stretch::geometry::Rect;
use stretch::geometry::Size;
use stretch::number::*;

use jni::objects::{GlobalRef, JObject, JString, JValue};
use jni::sys::{jlong, jobject};
use jni::JNIEnv;

pub struct JavaObject {
    instance: GlobalRef,
    env: JNIEnv<'static>,
}

fn rust_obj(env: &JNIEnv, j_obj: JObject) -> Box<JavaObject> {
    let ptr = env.get_field(j_obj, "rustPtr", "J").unwrap();
    unsafe { Box::from_raw(ptr.j().unwrap() as *mut JavaObject) }
}

impl JavaObject {
    fn new(env: JNIEnv<'static>, instance: JObject) -> Box<JavaObject> {
        Box::new(JavaObject { instance: env.new_global_ref(instance).unwrap(), env })
    }

    fn call_method(&self, name: &str, signature: &str, params: &[jni::objects::JValue]) -> JValue {
        self.env.call_method(self.instance.as_obj(), name, signature, params).unwrap()
    }
}

impl core::ShardViewManager for JavaObject {
    fn create_view(&self, context: &Any, kind: &str) -> Box<core::ShardView> {
        let kind = self.env.new_string(kind).unwrap();
        let context = context.downcast_ref::<GlobalRef>().unwrap();

        let j_view = self.call_method(
            "createView",
            "(Lapp/visly/shard/ShardContext;Ljava/lang/String;)Lapp/visly/shard/ShardView;",
            &[JValue::from(context.as_obj()), JValue::from(JObject::from(kind))],
        );
        rust_obj(&self.env, j_view.l().unwrap())
    }
}

impl core::ShardView for JavaObject {
    fn add_child(&mut self, child: &core::ShardView) {
        let child = child.as_any().downcast_ref::<JavaObject>().unwrap();
        self.call_method("addChild", "(Lapp/visly/shard/ShardView;)V", &[JValue::from(child.instance.as_obj())]);
    }

    fn set_prop(&mut self, key: &str, value: &JsonValue) {
        let key = self.env.new_string(key).unwrap();
        let value = self.env.new_string(value.dump()).unwrap();

        self.call_method(
            "setProp",
            "(Ljava/lang/String;Ljava/lang/String;)V",
            &[JValue::from(JObject::from(key)), JValue::from(JObject::from(value))],
        );
    }

    fn set_frame(&mut self, frame: Rect<f32>) {
        self.call_method(
            "setFrame",
            "(FFFF)V",
            &[JValue::from(frame.start), JValue::from(frame.end), JValue::from(frame.top), JValue::from(frame.bottom)],
        );
    }

    fn measure(&self, constraints: Size<Number>) -> Size<f32> {
        let size = self
            .call_method(
                "measure",
                "(FF)Lapp/visly/shard/Size;",
                &[
                    JValue::from(constraints.width.or_else(f32::NAN)),
                    JValue::from(constraints.height.or_else(f32::NAN)),
                ],
            )
            .l()
            .unwrap();

        let width = self.env.get_field(size, "width", "F").unwrap().f().unwrap();
        let height = self.env.get_field(size, "height", "F").unwrap().f().unwrap();

        Size { width, height }
    }

    fn as_any(&self) -> &Any {
        self
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_app_visly_shard_ShardViewManager_bind(env: JNIEnv<'static>, instance: JObject) -> jlong {
    Box::into_raw(JavaObject::new(env, instance)) as jlong
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_app_visly_shard_ShardViewManager_free(env: JNIEnv<'static>, instance: JObject) {
    let _view_manager = rust_obj(&env, instance);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_app_visly_shard_ShardViewManager_render(
    env: JNIEnv<'static>,
    instance: JObject,
    context: JObject,
    json: JString,
) -> jlong {
    let view_manager = rust_obj(&env, instance);
    let context = env.new_global_ref(context).unwrap();
    let json = env.get_string(json).unwrap();

    let root = core::render_root(Box::leak(view_manager), &context, json.to_str().unwrap());

    Box::into_raw(Box::new(root)) as jlong
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_app_visly_shard_ShardRoot_getView(env: JNIEnv<'static>, instance: JObject) -> jobject {
    let rust_ptr = env.get_field(instance, "rustPtr", "J").unwrap();
    let root = Box::from_raw(rust_ptr.j().unwrap() as *mut core::Root);
    let view = root.view_node.shard_view.as_any().downcast_ref::<JavaObject>().unwrap();
    let local = env.new_local_ref::<JObject>(view.instance.as_obj()).unwrap().into_inner();
    Box::leak(root);
    local
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_app_visly_shard_ShardRoot_measure(
    env: JNIEnv<'static>,
    instance: JObject,
    size: JObject,
) {
    let rust_ptr = env.get_field(instance, "rustPtr", "J").unwrap();
    let mut root = Box::from_raw(rust_ptr.j().unwrap() as *mut core::Root);

    let width = env.get_field(size, "width", "F").unwrap().f().unwrap();
    let height = env.get_field(size, "height", "F").unwrap().f().unwrap();

    root.measure(Size {
        width: if width.is_nan() { Number::Undefined } else { Number::Defined(width) },
        height: if height.is_nan() { Number::Undefined } else { Number::Defined(height) },
    });

    Box::leak(root);
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_app_visly_shard_ShardRoot_free(env: JNIEnv<'static>, instance: JObject) {
    let _root = rust_obj(&env, instance);
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_app_visly_shard_ShardView_bind(env: JNIEnv<'static>, instance: JObject) -> jlong {
    Box::into_raw(JavaObject::new(env, instance)) as jlong
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_app_visly_shard_ShardView_free(env: JNIEnv<'static>, instance: JObject) {
    let _shard_view = rust_obj(&env, instance);
}
