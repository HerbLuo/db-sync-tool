macro_rules! success {
    ($data:expr) => {
        rocket::serde::json::Json(crate::helper::resp::Data {
            ok: 1,
            data: $data,
        })
    };
}

macro_rules! fail {
    ($code:expr) => { fail!($code, ()) };

    ($code:expr,$debug:expr) => {{
        let serial = uuid::Uuid::new_v4().to_simple().to_string();
        log::error!("{}:{}:{:?}", serial, $code.code, $debug);
        crate::helper::resp::WithStatus (
            $code.status,
            rocket::serde::json::Json(crate::helper::resp::Data {
                ok: 0,
                data: crate::helper::resp::HttpError {
                    code: $code.code,
                    serial,
                    tip: Some($code.reason),
                },
            }),
        )
    }};
}
