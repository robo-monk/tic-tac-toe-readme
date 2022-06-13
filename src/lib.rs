extern crate cfg_if;
extern crate wasm_bindgen;
extern crate base64;

mod svg;
mod tictactoe;
mod utils;

use cfg_if::cfg_if;
use js_sys::{Array, ArrayBuffer, Function, JsString, Object, Reflect, Uint8Array};
use svg::{SVG, SVGTemplate};
use tictactoe::{Cell, State};
use wasm_bindgen::{prelude::*, JsCast, JsValue,};
use base64::{encode, decode};

use web_sys::{Request, Response, ResponseInit, UrlSearchParams, Headers};

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

impl ConsoleAdapter for JsString {
    fn to_console_obj(self) -> JsString {
        self
    }
}
impl ConsoleAdapter for &JsString {
    fn to_console_obj(self) -> JsString {
        self.to_owned() 
    }
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

    
    // fn redirect(url: &str) -> Result<Response, JsValue> {
    //     let mut res = ResponseInit::new();
    //     let headers = Headers::new().unwrap();
    //     headers.append("content-type", "image/svg+xml").expect("invalid headers");
    //     headers.append("location", url).expect("invalid headers");
    //     res.headers(&headers);
    //     res.status(301);
    //     Response::new_with_opt_str_and_init(Some(""), &res)
    // }

    fn redirect(url: &str) -> Result<Response, JsValue> {
        let mut res = ResponseInit::new();
        let headers = Headers::new().unwrap();
        headers.append("content-type", "text/html").expect("invalid headers");
        headers.append("cache-control", "no-cache, no-store, max-age=0, must-revalidate").expect("invalid headers");
        res.headers(&headers);
        res.status(200);
        let encoded_url = base64::encode(url);
        Response::new_with_opt_str_and_init(Some(&format!("<html><body><script> \
            let nanoid = (t=21)=>crypto.getRandomValues(new Uint8Array(t)).reduce(((t,e)=>t+=(e&=63)<36?e.toString(36):e<62?(e-26).toString(36).toUpperCase():e>62?'-':'_'),'');
            let hash = nanoid();
            console.log('hsh', hash)
            let url = new URL(atob(\"{}\"));
            url.searchParams.set('hash', hash);
            setTimeout(() => window.location.href = url.href, 1000);
         </script></body></html>", encoded_url)), &res)
    }
    fn respond(contents: &str, status: u16) -> Result<Response, JsValue> {
        let mut init = ResponseInit::new();
        init.status(status);
        Response::new_with_opt_str_and_init(Some(contents), &init)
    }

    fn respond_svg(svg: &str) -> Result<Response, JsValue> {
        let mut res = ResponseInit::new();
        let headers = Headers::new().unwrap();
        headers.append("content-type", "image/svg+xml").expect("invalid headers");
        headers.append("cache-control", "no-cache, no-store, max-age=0, must-revalidate").expect("invalid headers");
        res.headers(&headers);
        res.status(200);
        Response::new_with_opt_str_and_init(Some(svg), &res)
    }

    fn respond_json(json: &str, status: u16) -> Result<Response, JsValue> {
        respond(&format!("\"{}\"\n", json), status)
    }

    fn get_redis_key(params: UrlSearchParams) -> String {
        let username = params.get("u").unwrap_or_default();
        let index = params.get("i").unwrap_or_default();
        return format!("{}${}", username, index);
    }

    return match pathname.as_str() {
        "/api/click" => {
            let redirect_to = query_params.get("r").unwrap_or_default();

            let k = get_redis_key(query_params);
            let initial_state = kv.get_text(&k).await?.unwrap_or_default();
            log!("redis key", &k);
            let mut cell = Cell::new(&initial_state);
            let cell_before = cell.serialize();


            cell.state = match cell.state {
                State::Empty => State::X,
                State::X => State::O,
                _ => State::Empty,
            };

            let cell_after = &cell.serialize();

            kv.put_text(&k, &cell.serialize(), 24 * 60 * 60).await?;

            // let mut svg = SVG::new_from_template(SVGTemplate {});

            // respond_json(&format!("{} -> {}", &cell_before, &cell_after), 200)
            redirect(&redirect_to)
        }

        "/api/cell.svg" => {
            // let username = query_params.get("u").unwrap_or_default();
            // let index = query_params.get("i").unwrap_or_default();
            let k = get_redis_key(query_params);
            log!("redis key", &k);

            let state = kv.get_text(&k).await?.unwrap_or_default();
            let cell = Cell::new(&state);

            let svg_template = match cell.state {
                State::X => SVGTemplate::X,
                State::O => SVGTemplate::O,
                _ => SVGTemplate::Empty
                

            };
            let svg = SVG::new_from_template(svg_template); 
            respond_svg(&svg.render())
        }

        _ => {

            // let svg = SVG::new_from_template(SVGTemplate::o);

            // JsValue
            // respond(&svg.render(), 200)
            // respond_svg(&svg.render())
            // respond(&format!("{}", &contents), 200)
            // respond(contents);
            respond("These are not the droids you're looking for", 404)
        },
        // _ => respond("These are not the droids you're looking for", 404),
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
