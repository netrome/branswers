#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use rust_bert::pipelines::question_answering::{QaInput, QuestionAnsweringModel};
use rust_bert::pipelines::summarization::SummarizationModel;
use rust_bert::pipelines::ner::NERModel;
use rust_bert::pipelines::generation::LanguageGenerator;
use rust_bert::pipelines::generation::GPT2Generator;

use serde::Deserialize;
use rocket_contrib::json::Json;

#[derive(Deserialize)]
struct QAInput {
    question: String,
    context: String
}

//https://docs.rs/rust-bert/0.10.0/rust_bert/pipelines/index.html

                                                      
static FUCKING_INPUT: &'static str = "In findings published Tuesday in Cornell University's arXiv by a team of scientists \
from the University of Montreal and a separate report published Wednesday in Nature Astronomy by a team \
from University College London (UCL), the presence of water vapour was confirmed in the atmosphere of K2-18b, \
a planet circling a star in the constellation Leo. This is the first such discovery in a planet in its star's \
habitable zone — not too hot and not too cold for liquid water to exist. The Montreal team, led by Björn Benneke, \
used data from the NASA's Hubble telescope to assess changes in the light coming from K2-18b's star as the planet \
passed between it and Earth. They found that certain wavelengths of light, which are usually absorbed by water, \
weakened when the planet was in the way, indicating not only does K2-18b have an atmosphere, but the atmosphere \
contains water in vapour form. The team from UCL then analyzed the Montreal team's data using their own software \
and confirmed their conclusion. This was not the first time scientists have found signs of water on an exoplanet, \
but previous discoveries were made on planets with high temperatures or other pronounced differences from Earth. \
\"This is the first potentially habitable planet where the temperature is right and where we now know there is water,\" \
said UCL astronomer Angelos Tsiaras. \"It's the best candidate for habitability right now.\" \"It's a good sign\", \
said Ryan Cloutier of the Harvard–Smithsonian Center for Astrophysics, who was not one of either study's authors. \
\"Overall,\" he continued, \"the presence of water in its atmosphere certainly improves the prospect of K2-18b being \
a potentially habitable planet, but further observations will be required to say for sure. \"
K2-18b was first identified in 2015 by the Kepler space telescope. It is about 110 light-years from Earth and larger \
but less dense. Its star, a red dwarf, is cooler than the Sun, but the planet's orbit is much closer, such that a year \
on K2-18b lasts 33 Earth days. According to The Guardian, astronomers were optimistic that NASA's James Webb space \
telescope — scheduled for launch in 2021 — and the European Space Agency's 2028 ARIEL program, could reveal more \
about exoplanets like K2-18b.";


type ResultBox<T> = Result<T, Box<dyn std::error::Error>>;

fn main() -> ResultBox<()>{
    // let conciousness = std::fs::read_to_string("conciousness.txt").expect("Hello I am error!");
    // println!("asdfg: {:?}", question_answering("Amy is angry, but John is happy", "How does John feel?"));
    // println!("Summary: {:?}", summary(FUCKING_INPUT));
    // println!("Summary2: {:?}", summary(&conciousness));
    //named_entity_recognition( "My name is Amy. I live in Paris.");
    rocket::ignite().mount("/", routes![hello, ner, summary_route, generate_route, qa_route]).launch();
    Ok(())
}

fn question_answering(context: &str, question: &str) -> ResultBox<String> {
    let question = question.to_owned();
    let context = context.to_owned();
    let qa_model = QuestionAnsweringModel::new(Default::default())?;
    let answers = qa_model.predict(&vec![QaInput { question, context }], 1, 32);
    Ok(format!("Answers: {:?}", answers))
}

fn summary(text: &str) -> ResultBox<Vec<String>> {
    let summarization_model = SummarizationModel::new(Default::default())?;
    Ok(summarization_model.summarize(&[text]))
}

fn  named_entity_recognition(input: &str) -> ResultBox<String> {
    let ner_model = NERModel::new(Default::default())?;
    let output = ner_model.predict(&[input]);
    Ok(format!("{:?}", output))
}   

fn generation(text: &str) -> ResultBox<String>{
    let model = GPT2Generator::new(Default::default())?;
    let output = model.generate(Some(vec![text]), None);
    Ok(format!("{:?}", output))
}


// API -------------------------------------------------
#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[post("/ner", data="<text>")]
fn ner(text: String) -> String {
    let output = named_entity_recognition(&text);
    format!("{:?}", output)
}

#[post("/summary", data="<text>")]
fn summary_route(text: String) -> String {
    let output = summary(&text);
    format!("{:?}", output)
}

#[post("/gen", data="<text>")]
fn generate_route(text: String) -> String {
    let output = generation(&text);
    format!("{:?}", output)
}

#[post("/qa", data="<input>")]
fn qa_route(input: Json<QAInput>) -> String {
    let output = question_answering(&input.context, &input.question);
    format!("{:?}", output)
}
