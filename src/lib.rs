extern crate cfg_if;
extern crate wasm_bindgen;

mod svg;
mod tictactoe;
mod utils;

use cfg_if::cfg_if;
use js_sys::{Array, ArrayBuffer, Function, JsString, Object, Reflect, Uint8Array};
use svg::{SVG, SVGTemplate};
use tictactoe::{Cell, State};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Request, Response, ResponseInit, UrlSearchParams};

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

trait ConsoleAdapter {
    fn to_console_obj(self) -> JsString;
}

impl ConsoleAdapter for &str {
    fn to_console_obj(self) -> JsString {
        JsString::from(self)
        // string
    }
}

impl ConsoleAdapter for String {
    fn to_console_obj(self) -> JsString {
        JsString::from(self)
    }
}

impl ConsoleAdapter for Vec<&str> {
    fn to_console_obj(self) -> JsString {
        self.join(" ").to_console_obj()
    }
}

#[wasm_bindgen]
pub async fn handle(kv: WorkersKvJs, req: JsValue, log: JsValue) -> Result<Response, JsValue> {
    let log: Function = log.dyn_into()?;
    macro_rules! log {
        ($s: expr) => {
            // let string = JsString::from($s);
            log.call1(&log, &($s.to_console_obj()))?
        };
        ($($x:expr),*) => (
            log!(vec![$($x),*])
        );
    }

    // log!( "yo", "does", "this", "work??" );

    let req: Request = req.dyn_into()?;

    let url = web_sys::Url::new(&req.url())?;
    let pathname = url.pathname();
    let query_params = url.search_params();
    let kv = WorkersKv { kv };

    log!("[>] incoming", &url.host(), "/", &pathname, "\n");

    fn respond(json: &str, status: u16) -> Result<Response, JsValue> {
        let mut init = ResponseInit::new();
        init.status(status);
        Response::new_with_opt_str_and_init(Some(&format!("\"{}\"\n", json)), &init)
    }

    fn get_redis_key(params: UrlSearchParams) -> String {
        let username = params.get("u").unwrap_or_default();
        let index = params.get("i").unwrap_or_default();
        return format!("{}${}", username, index);
    }

    return match pathname.as_str() {
        "/api/click" => {
            let k = get_redis_key(query_params);
            let initial_state = kv.get_text(&k).await?.unwrap_or_default();
            log!("redis key", &k);
            let mut cell = Cell::new(&initial_state);
            let cell_before = cell.serialize();

            cell.state = match cell.state {
                State::Empty => State::X,
                _ => State::Empty,
            };

            let cell_after = &cell.serialize();

            kv.put_text(&k, &cell.serialize(), 24 * 60 * 60).await?;

            // let mut svg = SVG::new_from_template(SVGTemplate {});

            respond(&format!("{} -> {}", &cell_before, &cell_after), 200)
        }

        "/api/cell.svg" => {
            // let username = query_params.get("u").unwrap_or_default();
            // let index = query_params.get("i").unwrap_or_default();
            let k = get_redis_key(query_params);
            log!("redis key", &k);

            let state = kv.get_text(&k).await?.unwrap_or_default();
            respond(&state, 200)
        }

        _ => respond("These are not the droids you're looking for", 404),
    };

    match req.method().as_str() {
        "GET" => {
            let value = kv.get_text(&pathname).await?.unwrap_or_default();
            let mut init = ResponseInit::new();
            init.status(200);
            Response::new_with_opt_str_and_init(Some(&format!("\"{}\"\n", value)), &init)
        }
        "PUT" => {
            let value = query_params.get("value").unwrap_or_default();
            // set a TTL of 60 seconds:
            kv.put_text(&pathname, &value, 60).await?;
            let mut init = ResponseInit::new();
            init.status(200);
            Response::new_with_opt_str_and_init(None, &init)
        }
        _ => {
            let mut init = ResponseInit::new();
            init.status(400);
            Response::new_with_opt_str_and_init(None, &init)
        }
    }
}

#[wasm_bindgen]
extern "C" {
    pub type WorkersKvJs;

    #[wasm_bindgen(structural, method, catch)]
    pub async fn put(
        this: &WorkersKvJs,
        k: JsValue,
        v: JsValue,
        options: JsValue,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(structural, method, catch)]
    pub async fn get(
        this: &WorkersKvJs,
        key: JsValue,
        options: JsValue,
    ) -> Result<JsValue, JsValue>;
}

struct WorkersKv {
    kv: WorkersKvJs,
}

impl WorkersKv {
    async fn put_text(&self, key: &str, value: &str, ttl: u64) -> Result<(), JsValue> {
        let options = Object::new();
        Reflect::set(&options, &"expirationTtl".into(), &(ttl as f64).into())?;
        self.kv
            .put(JsValue::from_str(key), value.into(), options.into())
            .await?;
        Ok(())
    }

    async fn put_vec(&self, key: &str, value: &[u8], ttl: u64) -> Result<(), JsValue> {
        let options = Object::new();
        Reflect::set(&options, &"expirationTtl".into(), &(ttl as f64).into())?;
        let typed_array = Uint8Array::new_with_length(value.len() as u32);
        typed_array.copy_from(value);
        self.kv
            .put(
                JsValue::from_str(key),
                typed_array.buffer().into(),
                options.into(),
            )
            .await?;
        Ok(())
    }

    async fn get_text(&self, key: &str) -> Result<Option<String>, JsValue> {
        let options = Object::new();
        Reflect::set(&options, &"type".into(), &"text".into())?;
        Ok(self
            .kv
            .get(JsValue::from_str(key), options.into())
            .await?
            .as_string())
    }

    async fn get_vec(&self, key: &str) -> Result<Option<Vec<u8>>, JsValue> {
        let options = Object::new();
        Reflect::set(&options, &"type".into(), &"arrayBuffer".into())?;
        let value = self.kv.get(JsValue::from_str(key), options.into()).await?;
        if value.is_null() {
            Ok(None)
        } else {
            let buffer = ArrayBuffer::from(value);
            let typed_array = Uint8Array::new_with_byte_offset(&buffer, 0);
            let mut v = vec![0; typed_array.length() as usize];
            typed_array.copy_to(v.as_mut_slice());
            Ok(Some(v))
        }
    }
}

// use serde_json::json;
// use worker::*;

// // mod utils;

// extern crate cfg_if;
// extern crate wasm_bindgen;

// mod utils;

// use cfg_if::cfg_if;
// // use js_sys::{ArrayBuffer, Object, Reflect, Uint8Array};
// use js_sys::{ArrayBuffer, Object, Reflect, Uint8Array};
// use wasm_bindgen::{prelude::*, JsCast};
// use web_sys::{Request, Response, ResponseInit};

// cfg_if! {
//     // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
//     // allocator.
//     if #[cfg(feature = "wee_alloc")] {
//         extern crate wee_alloc;
//         #[global_allocator]
//         static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
//     }
// }

// #[wasm_bindgen]
// extern "C" {
//     pub type WorkersKvJs;

//     #[wasm_bindgen(structural, method, catch)]
//     pub async fn put(
//         this: &WorkersKvJs,
//         k: JsValue,
//         v: JsValue,
//         options: JsValue,
//     ) -> Result<JsValue, JsValue>;

//     #[wasm_bindgen(structural, method, catch)]
//     pub async fn get(
//         this: &WorkersKvJs,
//         key: JsValue,
//         options: JsValue,
//     ) -> Result<JsValue, JsValue>;
// }

// struct WorkersKv {
//     kv: WorkersKvJs,
// }

// // async fn put_text(&self, key: &str, value: &str, expiration_ttl: u64) -> Result<(), JsValue> {

// impl WorkersKv {
//     async fn put_text(&self, key: &str, value: &str, ttl: u64) -> Result<(), JsValue> {
//         let options = Object::new();
//         Reflect::set(&options, &"expirationTtl".into(), &(ttl as f64).into())?;
//         // let ary = Uint8Array::new_with_length(value.len() as u32);
//         // ary.copy_from(value);

//         self.kv
//             // .put(JsValue::from_str(key), ary.buffer().into(), options.into())
//             .put(JsValue::from_str(key), value.into(), options.into())
//             .await?;

//         Ok(())
//     }

//     async fn get_text(&self, key: &str) -> Result<Option<String>, JsValue> {
//         let options = Object::new();
//         Reflect::set(&options, &"type".into(), &"text".into())?;

//         Ok(self
//             .kv
//             .get(JsValue::from_str(key), options.into())
//             .await?
//             .as_string())
//     }
// }

// #[wasm_bindgen]
// pub async fn handle(kv: WorkersKvJs, req: JsValue) -> Result<Response, JsValue> {
//     let req: Request = req.dyn_into()?;
//     let url = web_sys::Url::new(&req.url())?;
//     let pathname = url.pathname();
//     let query = url.search_params();
//     let kv = WorkersKv { kv };

//     log_request(&req);
//     utils::set_panic_hook();

//     let value = kv.get_text(&pathname).await?.unwrap_or_default();
//     let mut res = ResponseInit::new();
//     res.status(200);
//     Response::new_with_opt_str_and_init(Some(&format!("\"{}\"\n", value)), &init)
// }

// async fn get_icon(&req: Request) {

//               let value = kv.get_text(&pathname).await?.unwrap_or_default();

// }
