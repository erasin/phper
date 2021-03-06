use crate::{
    errors::HttpServerError, request::HTTP_REQUEST_CLASS_NAME, response::HTTP_RESPONSE_CLASS_NAME,
    utils::replace_and_get,
};
use hyper::{
    server::{conn::AddrIncoming, Builder},
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use phper::{
    classes::{ClassEntry, DynamicClass, StatelessClassEntry, Visibility},
    functions::Argument,
    values::Val,
};
use std::{convert::Infallible, mem::replace, net::SocketAddr, sync::Arc};
use tokio::{runtime::Handle, sync::Mutex};

const HTTP_SERVER_CLASS_NAME: &'static str = "HttpServer\\HttpServer";

pub fn make_server_class() -> DynamicClass<Option<Builder<AddrIncoming>>> {
    let mut class = DynamicClass::new_with_default(HTTP_SERVER_CLASS_NAME);

    class.add_property("host", Visibility::Private, "127.0.0.1");
    class.add_property("port", Visibility::Private, 8080);
    class.add_property("onRequestHandle", Visibility::Private, ());

    class.add_method(
        "__construct",
        Visibility::Public,
        |this, arguments| {
            let host = arguments[0].as_string()?;
            let port = arguments[1].as_long()?;
            this.set_property("host", Val::new(&*host));
            this.set_property("port", Val::new(port));
            let addr = format!("{}:{}", host, port).parse::<SocketAddr>()?;
            let builder = Server::bind(&addr);
            *this.as_mut_state() = Some(builder);
            Ok::<_, HttpServerError>(())
        },
        vec![Argument::by_val("host"), Argument::by_val("port")],
    );

    class.add_method(
        "onRequest",
        Visibility::Public,
        |this, arguments| {
            this.set_property("onRequestHandle", Val::new(arguments[0].duplicate()?));
            Ok::<_, phper::Error>(())
        },
        vec![Argument::by_val("handle")],
    );

    class.add_method(
        "start",
        Visibility::Public,
        |this, _| {
            let builder = replace(this.as_mut_state(), None).unwrap();
            let handle = this
                .duplicate_property("onRequestHandle")
                .map_err(phper::Error::NotRefCountedType)?;
            let handle = Arc::new(Mutex::new(handle));

            let make_svc = make_service_fn(move |_conn| {
                let handle = handle.clone();

                async move {
                    Ok::<_, Infallible>(service_fn(move |_: Request<Body>| {
                        let handle = handle.clone();
                        async move {
                            let handle = handle.lock().await;

                            let request =
                                StatelessClassEntry::from_globals(HTTP_REQUEST_CLASS_NAME)
                                    .unwrap()
                                    .new_object([])
                                    .unwrap();
                            let request = Val::new(request);
                            let mut response = ClassEntry::<Response<Body>>::from_globals(
                                HTTP_RESPONSE_CLASS_NAME,
                            )
                            .unwrap()
                            .new_object([])
                            .unwrap();
                            let response_val = response.duplicate();
                            let response_val = Val::new(response_val);
                            handle
                                .call([request, response_val])
                                .map_err(phper::Error::CallFunction)
                                .unwrap();

                            let response = replace_and_get(response.as_mut_state());
                            Ok::<Response<Body>, Infallible>(response)
                        }
                    }))
                }
            });

            let server = builder.serve(make_svc);
            Handle::current().block_on(server)?;

            Ok::<_, HttpServerError>(())
        },
        vec![],
    );

    class
}
