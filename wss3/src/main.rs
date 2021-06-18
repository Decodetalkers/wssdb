use {

    ws::{
        listen, 
        Handler, 
        Message, 
        Request, 
        Response, 
        Result, 
        Sender
    },
    database
};
// This can be read from a file
static INDEX_HTML: &'static [u8] = br#"
<!DOCTYPE html>
<html>
 <head>
  <meta charset="utf-8">
 </head>
 <body>
      <pre id="messages"></pre>
   <form id="form">
    <input type="text" id="msg">
    <input type="submit" value="Send">
   </form>
      <script>
        var socket = new WebSocket("ws://" + window.location.host + "/ws");
        socket.onmessage = function (event) {
          var messages = document.getElementById("messages");
          messages.innerHTML=event.data;
        };
        var form = document.getElementById("form");
        form.addEventListener('submit', function (event) {
          event.preventDefault();
          var input = document.getElementById("msg");
          socket.send(input.value);
          input.value = "";
        });
  </script>
 </body>
</html>
    "#;

// Server web application handler
struct Server {
    out: Sender,
}
impl Server {
    #[allow(dead_code)]
    fn response(&mut self){
        let names = database::first().unwrap();
        let mut name :String = String::from("{\"sites\": [ ");
        for na in names {
            //println!("{}",na);
            //self.response(na);
            name=name+&na+",\n";
        }
        name+="{\"id\":\"欢迎留言\",\"data\":\"欢迎留言\"}";
        name+="]}";
        //println!("{}",name);
        let f = self.out.broadcast(name);
        match f {
            Ok(_) => () ,
            Err(error) => {
                panic!("Something happend {:?}",error)
            },
        };
    }
}
impl Handler for Server {
    //
    #[allow(unused_parens)]
    fn on_request(&mut self, req: &Request) -> Result<(Response)> {
        // Using multiple handlers is better (see router example)
        match req.resource() {
            // The default trait implementation
            "/ws" => {
                self.response();
                Response::from_request(req)
            },

            // Create a custom response
            "/" => {
                {
                    println!("ssss");
                    Ok(Response::new(200, "OK", INDEX_HTML.to_vec()))
                }
            },

            _ => Ok(Response::new(404, "Not Found", b"404 - Not Found".to_vec())),
        }
    }

    // Handle messages recieved in the websocket (in this case, only on /ws)

    fn on_message(&mut self, msg: Message) -> Result<()> {
        // Broadcast to all connections
        let names = database::data(msg.to_string()).unwrap();
        let mut name :String = String::from("{\"sites\": [ ");
        for na in names.iter().rev() {
            //println!("{}",na);
            //self.response(na);
            name=name+&na+",\n";
        }
        name+="{\"id\":\"欢迎留言\",\"data\":\"欢迎留言\"}";
        name+="]}";
        //println!("{}",name);
        self.out.broadcast(name)
        //self.response();

    }
}

fn main() {
    // Listen on an address and call the closure for each connection
    listen("127.0.0.1:8080", |out| Server { out }).unwrap()
}
