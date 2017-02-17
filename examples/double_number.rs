extern crate iron;
extern crate alexa;
use iron::prelude::*;

struct RequestHandler{}
impl alexa::RequestHandler for RequestHandler {
    fn handle_request(&self, req: &alexa::Request) -> alexa::Response {
        match req.body {
            alexa::RequestBody::IntentRequest(ref ir) => {
                match ir.name.as_str() {
                    "DoubleNumber" => {
                        let num_o: Option<f64> = ir.slots.get("num").and_then(|n| n.replace(",", match req.locale() {"de-DE" => ".", _ => ",",}).parse().ok());
                        match num_o {
                            Some(num) => doubled_number_response(num, req.locale()),
                            None => i_dont_understand(req.locale()),
                        }
                    },
                    _ => i_dont_understand(req.locale()),
                }
            },
            _ => i_dont_understand(req.locale()),
        }
    }
}
fn doubled_number_response<'a>(num: f64, locale: &str) -> alexa::Response<'a> {
        alexa::Response {
            session_attributes: None,
            card: None,
            reprompt: None,
            output_speech:
                match locale {
                    "de-DE" => Some(alexa::OutputSpeech::Text(format!("Zweimal {} ist {}", num, num * 2f64).into())),
                    _ => Some(alexa::OutputSpeech::Text(format!("Double {} is {}", num, num * 2f64).into())),
                },
            should_end_session: true,
        }
}
fn i_dont_understand<'a>(locale: &str) -> alexa::Response<'a> {
        alexa::Response {
            session_attributes: None,
            card: None,
            reprompt: None,
            output_speech:
                match locale {
                    "de-DE" => Some(alexa::OutputSpeech::Text("Oh nein, ich verstehe nicht, was Du sagst!".into())),
                    _ => Some(alexa::OutputSpeech::Text("Oh no, I don't understand what you said!".into())),
                },
            should_end_session: true,
        }
}
fn main() {
    let rh = RequestHandler{};
    let ih = alexa::IronHandler::new("amzn1.ask.skill.a4c09e11-72b8-4b5d-a2dc-d674fa717f14".to_owned(),Box::new(rh));
    let chain = Chain::new(ih);
    Iron::new(chain).http("0.0.0.0:3000").unwrap();
}
