use popper::interpreter::Interpreter;
use popper::value::{Implementation, Object, Value};
use popper::create;
use popper::define_function;
use popper::register_stdlib;
use popper::function;

use verso::{Server, RequestBody, Router, Response};
use popper::value::Type;

trait RouterStdLib {
    function!(new);
    function!(get);
    function!(post);
    function!(put);
    function!(delete);
}

trait RequestStdLib {
    function!(header);
    function!(url_query);
    function!(post_info);
}

trait ResponseStdLib {
    function!(new);
    function!(set_header);
    function!(set_body);
}

trait ServerStdLib {
    function!(start);
}




struct HttpServer {
    server: Server,
}

struct HttpRouter<'a> {
    router: Router<'a>,
}

impl HttpServer {
    fn create_object(&self) -> Object {
        Object {
            value: Value::BuiltinStruct,
            implementations: vec![
                Implementation::Get(Rc::new(self))
            ],
            type_: Type::Struct("HttpServer".to_string()),
            tags: std::default::Default::default()
        }
    }
}

define_function!(new() {
    let server = Server::new();
    let http_server = HttpServer {
        server
    };
    http_server.create_object()
}, function_name = "new");

impl Into<HttpRouter> for Object {
    fn into(self) -> HttpRouter {
        match self.value {
            Value::BuiltinStruct(b) => {
                match b.downcast_ref::<HttpRouter>() {
                    Some(r) => r.clone(),
                    None => panic!("Cannot downcast to HttpRouter")
                }
            },
        }
    }
}