pub fn route_assets() -> axum::Router {
    let assets = macro_log::read_dir!("web/toast-web/dist");
    let mut router = axum::Router::new();
    for (path, bin) in assets {
        #[cfg(windows)]
        let path = format!("/{}", path.replace("\\", "/"));
        macro_log::d!("serve: {path}");
        let mime = match () {
            _ if path.ends_with(".html") => axum_extra::response::TypedHeader(axum_extra::headers::ContentType::html()),
            _ if path.ends_with(".css") => axum_extra::response::TypedHeader(axum_extra::headers::ContentType::from(mime::TEXT_CSS)),
            _ if path.ends_with(".js") => axum_extra::response::TypedHeader(axum_extra::headers::ContentType::from(mime::TEXT_JAVASCRIPT)),
            _ if path.ends_with(".json") => axum_extra::response::TypedHeader(axum_extra::headers::ContentType::from(mime::APPLICATION_JSON)),
            _ if path.ends_with(".svg") => axum_extra::response::TypedHeader(axum_extra::headers::ContentType::from(mime::IMAGE_SVG)),
            _ => axum_extra::response::TypedHeader(axum_extra::headers::ContentType::octet_stream()),
        };
        let handler = || async {
            (mime, bin.as_ref())
        };
        if path.ends_with("index.html") {
            let path = &path[.. path.len() - "index.html".len()];
            router = router.route(&path, axum::routing::get(handler.clone()));
        }
        router = router.route(&path, axum::routing::get(handler));
    }
    router
}