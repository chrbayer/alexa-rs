extern crate iron;
extern crate bodyparser;
extern crate alexa;
use iron::prelude::*;

struct RequestHandler;

impl alexa::RequestHandler for RequestHandler {
    fn handle_request(&self, req: &alexa::Request) -> alexa::Response {
        match req.body {
            alexa::RequestBody::LaunchRequest => {
                welcome(req.locale())
            },
            alexa::RequestBody::IntentRequest(ref ir) => {
                match ir.name.as_str() {
                    "DoubleNumber" => {
                        let num_o: Option<f64> = ir.slots.get("num").and_then(|n| n.replace(",", match req.locale() {"de-DE" => ".", _ => ",",}).parse().ok());
                        match num_o {
                            Some(num) => doubled_number_response(num, req.locale(), req.session_new()),
                            None => i_dont_understand(req.locale(), req.session_new()),
                        }
                    },
                    _ => i_dont_understand(req.locale(), req.session_new()),
                }
            },
            alexa::RequestBody::SessionEndedRequest(ref _ser) => {
                session_ended()
            },
        }
    }
}

fn welcome<'a>(locale: &str) -> alexa::Response<'a> {
    alexa::Response {
        session_attributes: None,
        card: None,
        reprompt: None,
        output_speech:
        match locale {
            "de-DE" => Some(alexa::OutputSpeech::Text("Willkommen zum grandiosen Verdoppler!".into())),
            _ => Some(alexa::OutputSpeech::Text("Welcome to double number!".into())),
        },
        should_end_session: false,
    }
}

fn doubled_number_response<'a>(num: f64, locale: &str, new_session: bool) -> alexa::Response<'a> {
        alexa::Response {
            session_attributes: None,
            card: None,
            reprompt: None,
            output_speech:
                match locale {
                    "de-DE" => Some(alexa::OutputSpeech::Text(format!("Zweimal {} ist {}", num, num * 2f64).into())),
                    _ => Some(alexa::OutputSpeech::Text(format!("Double {} is {}", num, num * 2f64).into())),
                },
            should_end_session: new_session,
        }
}

fn session_ended<'a>() -> alexa::Response<'a> {
    alexa::Response {
        session_attributes: None,
        card: None,
        reprompt: None,
        output_speech: None,
        should_end_session: true,
    }
}

fn i_dont_understand<'a>(locale: &str, new_session: bool) -> alexa::Response<'a> {
        alexa::Response {
            session_attributes: None,
            card: None,
            reprompt: None,
            output_speech:
                match locale {
                    "de-DE" => Some(alexa::OutputSpeech::Text("Oh nein, ich verstehe nicht, was Du sagst!".into())),
                    _ => Some(alexa::OutputSpeech::Text("Oh no, I don't understand what you said!".into())),
                },
            should_end_session: new_session,
        }
}

fn log_body(req: &mut Request) -> IronResult<()> {
    let body = req.get::<bodyparser::Raw>();
    match body {
        Ok(Some(body)) => println!("Read body:\n{}", body),
        Ok(None) => println!("No body"),
        Err(err) => println!("Error: {:?}", err)
    }

    Ok(())
}


fn main() {
    let rh = RequestHandler{};
    let ih = alexa::IronHandler::new("amzn1.ask.skill.a4c09e11-72b8-4b5d-a2dc-d674fa717f14".to_owned(), Box::new(rh));
    let mut chain = Chain::new(ih);
    chain.link_before(log_body);
    Iron::new(chain).http("0.0.0.0:3000").unwrap();
}
