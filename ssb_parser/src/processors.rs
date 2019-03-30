// Imports
use super::data::Ssb;
use pest_derive::Parser;    // Macro
use pest::Parser;   // Trait

// PEG parser
#[derive(Parser)]
#[grammar = "ssb.pest"]
struct SsbPegParser;

// Stream parser for ssb data
pub struct SsbParser {
    data: Ssb
}
impl SsbParser {
    // Constructor
    pub fn new(script: &str) -> Self {
        // Parse script and panic on fail
        let pairs = SsbPegParser::parse(Rule::script, script).unwrap_or_else(|e| panic!("{}", e));
        // Iterate through section entries
        for section_entry_pair in pairs {
            match section_entry_pair.as_rule() {
                // Meta entry
                Rule::meta_entry => for meta_entry_pair in section_entry_pair.into_inner() {
                    match meta_entry_pair.as_rule() {
                        // Meta entry key
                        Rule::meta_entry_key => {
                            println!("Meta key: {}", meta_entry_pair.as_str());
                        }
                        // Meta entry value
                        Rule::meta_entry_value => {
                            println!("Meta value: {}", meta_entry_pair.as_str());
                        }
                        // Nothing more in this scope
                        _ => unreachable!()
                    }
                }
                // Frame entry
                Rule::frame_entry => for frame_entry_pair in section_entry_pair.into_inner() {
                    match frame_entry_pair.as_rule() {
                        // Frame entry key
                        Rule::frame_entry_key => {
                            println!("Frame key: {}", frame_entry_pair.as_str());
                        }
                        // Frame entry value
                        Rule::frame_entry_value => {
                            println!("Frame value: {}", frame_entry_pair.as_str());
                        }
                        // Nothing more in this scope
                        _ => unreachable!()
                    }
                }
                // "End of input" not of interest
                Rule::EOI => (),
                // Nothing more in this scope
                _ => unreachable!()
            }
        }
        // Return instance
        Self {
            data: Ssb {}
        }
    }
}