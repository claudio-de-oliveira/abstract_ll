use std::collections::HashMap;

use crate::grammar::{self, AddOpType, DecimalToken, FunctionToken, IntegerToken, LiteralToken, MulOpToken, MulOpType, RelOpToken, RelOpType, UnknowToken, VariableToken };
use crate::tag::Tag;
use crate::token::{ComplementTrait, SimpleToken, ValuedToken};
use crate::variable::Variable;
use crate::{scanner_environment::ScannerEnv, token::Token};

pub trait Scanner {
    fn new() -> Self;
    fn next_token(&self, env : &mut ScannerEnv, symbol_table : &HashMap<String, Box<dyn Token>>, variables : &HashMap<String, Variable>) -> Box<dyn Token>;
}

pub struct CeScanner {
    pub reserved_words: HashMap<&'static str, Tag>
}

// impl fmt::Display for CeScanner {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:#?}", self.reserved_words)
//     }
// }

impl Scanner for CeScanner {

    fn new() -> Self {
        let reserved_words = HashMap::from([
            ("alt", grammar::VT_ALT),
            ("and", grammar::VT_AND),
            ("answered", grammar::VT_ANSWERED),
            ("as", grammar::VT_AS),
            ("attach", grammar::VT_ATTACH),
            ("attribute", grammar::VT_ATTRIBUTE),
            ("authornote", grammar::VT_AUTHORNOTE),
            ("by", grammar::VT_BY),
            ("capitals", grammar::VT_CAPITALS),
            ("cell", grammar::VT_CELL),
            ("cloaked", grammar::VT_CLOAKED),
            ("collect", grammar::VT_COLLECT),
            ("collectvalues", grammar::VT_COLLECTVALUES),
            ("committed", grammar::VT_COMMITTED),
            ("datatype", grammar::VT_DATATYPE),
            ("deferred", grammar::VT_DEFERRED),
            ("definite", grammar::VT_DEFINITE),
            ("doctitle", grammar::VT_DOCTITLE),
            ("document", grammar::VT_DOCUMENT),
            ("else", grammar::VT_ELSE),
            ("every", grammar::VT_EVERY),
            ("exists", grammar::VT_EXISTS),
            ("export", grammar::VT_EXPORT),
            ("expressiontext", grammar::VT_EXPRESSIONTEXT),
            ("false", grammar::VT_FALSE),
            ("foreach", grammar::VT_FOREACH),
            ("format", grammar::VT_FORMAT),
            ("from", grammar::VT_FROM),
            ("hyperlink", grammar::VT_HYPERLINK),
            ("and", grammar::VT_AND),
            ("if", grammar::VT_IF),                           // test that a value equals another value
            ("ifknownelse", grammar::VT_IFKNOWNELSE),
            ("include", grammar::VT_INCLUDE),
            // ("is", grammar::VT_RELOP),                       // test that a value equals another value
            // ("isatleast", grammar::VT_RELOP),                 // test that a value is more than or equal to another value
            // ("isatmost", grammar::VT_RELOP),                 // test that a value is less than or equal to another value
            // ("islessthan", grammar::VT_RELOP),               // test that a value is less than another value
            // ("ismorethan", grammar::VT_RELOP),               // test that a value is more than another value
            ("isnot", grammar::VT_RELOP),                    // test that a value is not equal to another value
            ("known", grammar::VT_KNOWN),
            ("knowntrue", grammar::VT_KNOWNTRUE),
            ("label", grammar::VT_LABEL),
            ("list", grammar::VT_LIST),
            ("lower", grammar::VT_LOWER),
            ("mark", grammar::VT_MARK),
            ("nonmutualand", grammar::VT_NONMUTUALAND),
            ("nonmutualor", grammar::VT_NONMUTUALOR),
            ("nonrepeated", grammar::VT_NONREPEATED),
            ("not", grammar::VT_NOT),
            ("note", grammar::VT_NOTE),
            ("now", grammar::VT_NOW),
            ("occurrence", grammar::VT_OCCURRENCE),
            ("onlyoninput", grammar::VT_ONLYONINPUT),
            ("onlyonoutput", grammar::VT_ONLYONOUTPUT),
            ("onlyother", grammar::VT_ONLYOTHER),
            ("or", grammar::VT_OR),
            ("otherselections", grammar::VT_OTHERSELECTIONS),
            ("picture", grammar::VT_PICTURE),
            ("prefix", grammar::VT_PREFIX),
            ("prescribedselections", grammar::VT_PRESCRIBEDSELECTIONS),
            ("proper", grammar::VT_PROPER),
            ("punctuation", grammar::VT_PUNCTUATION),
            ("ref", grammar::VT_REF),
            ("reference", grammar::VT_REFERENCE),
            ("relevance", grammar::VT_RELEVANCE),
            ("answered", grammar::VT_ANSWERED),
            ("alt", grammar::VT_ALT),
            ("and", grammar::VT_AND),
            ("answered", grammar::VT_ANSWERED),
            ("as", grammar::VT_AS),
            ("attach", grammar::VT_ATTACH),
            ("attribute", grammar::VT_ATTRIBUTE),
            ("authornote", grammar::VT_AUTHORNOTE),
            ("by", grammar::VT_BY),
            ("capitals", grammar::VT_CAPITALS),
            ("cell", grammar::VT_CELL),
            ("cloaked", grammar::VT_CLOAKED),
            ("collect", grammar::VT_COLLECT),
            ("collectvalues", grammar::VT_COLLECTVALUES),
            ("committed", grammar::VT_COMMITTED),
            ("datatype", grammar::VT_DATATYPE),
            ("deferred", grammar::VT_DEFERRED),
            ("definite", grammar::VT_DEFINITE),
            ("doctitle", grammar::VT_DOCTITLE),
            ("document", grammar::VT_DOCUMENT),
            ("else", grammar::VT_ELSE),
            ("every", grammar::VT_EVERY),
            ("exists", grammar::VT_EXISTS),
            ("export", grammar::VT_EXPORT),
            ("expressiontext", grammar::VT_EXPRESSIONTEXT),
            ("false", grammar::VT_FALSE),
            ("foreach", grammar::VT_FOREACH),
            ("format", grammar::VT_FORMAT),
            ("from", grammar::VT_FROM),
            ("hyperlink", grammar::VT_HYPERLINK),
            ("as", grammar::VT_AS),
            ("attach", grammar::VT_ATTACH),
            ("attribute", grammar::VT_ATTRIBUTE),
            ("authornote", grammar::VT_AUTHORNOTE),
            ("by", grammar::VT_BY),
            ("capitals", grammar::VT_CAPITALS),
            ("cell", grammar::VT_CELL),
            ("cloaked", grammar::VT_CLOAKED),
            ("collect", grammar::VT_COLLECT),
            ("collectvalues", grammar::VT_COLLECTVALUES),
            ("committed", grammar::VT_COMMITTED),
            ("datatype", grammar::VT_DATATYPE),
            ("deferred", grammar::VT_DEFERRED),
            ("definite", grammar::VT_DEFINITE),
            ("doctitle", grammar::VT_DOCTITLE),
            ("document", grammar::VT_DOCUMENT),
            ("else", grammar::VT_ELSE),
            ("every", grammar::VT_EVERY),
            ("exists", grammar::VT_EXISTS),
            ("export", grammar::VT_EXPORT),
            ("expressiontext", grammar::VT_EXPRESSIONTEXT),
            ("false", grammar::VT_FALSE),
            ("foreach", grammar::VT_FOREACH),
            ("format", grammar::VT_FORMAT),
            ("from", grammar::VT_FROM),
            ("hyperlink", grammar::VT_HYPERLINK),
            ("as", grammar::VT_AS),
            ("attach", grammar::VT_ATTACH),
            ("attribute", grammar::VT_ATTRIBUTE),
            ("authornote", grammar::VT_AUTHORNOTE),
            ("by", grammar::VT_BY),
            ("capitals", grammar::VT_CAPITALS),
            ("cell", grammar::VT_CELL),
            ("cloaked", grammar::VT_CLOAKED),
            ("collect", grammar::VT_COLLECT),
            ("collectvalues", grammar::VT_COLLECTVALUES),
            ("committed", grammar::VT_COMMITTED),
            ("datatype", grammar::VT_DATATYPE),
            ("deferred", grammar::VT_DEFERRED),
            ("definite", grammar::VT_DEFINITE),
            ("doctitle", grammar::VT_DOCTITLE),
            ("document", grammar::VT_DOCUMENT),
            ("else", grammar::VT_ELSE),
            ("every", grammar::VT_EVERY),
            ("exists", grammar::VT_EXISTS),
            ("export", grammar::VT_EXPORT),
            ("expressiontext", grammar::VT_EXPRESSIONTEXT),
            ("false", grammar::VT_FALSE),
            ("foreach", grammar::VT_FOREACH),
            ("format", grammar::VT_FORMAT),
            ("from", grammar::VT_FROM),
            ("hyperlink", grammar::VT_HYPERLINK),
            ("if", grammar::VT_IF),                           // test that a value equals another value
            ("ifknownelse", grammar::VT_IFKNOWNELSE),
            ("include", grammar::VT_INCLUDE),
            ("is", grammar::VT_RELOP),                       // test that a value equals another value
            ("isatleast", grammar::VT_RELOP),                 // test that a value is more than or equal to another value
            ("isatmost", grammar::VT_RELOP),                 // test that a value is less than or equal to another value
            ("islessthan", grammar::VT_RELOP),               // test that a value is less than another value
            ("ismorethan", grammar::VT_RELOP),               // test that a value is more than another value
            ("isnot", grammar::VT_RELOP),                    // test that a value is not equal to another value
            ("known", grammar::VT_KNOWN),
            ("knowntrue", grammar::VT_KNOWNTRUE),
            ("label", grammar::VT_LABEL),
            ("list", grammar::VT_LIST),
            ("lower", grammar::VT_LOWER),
            ("mark", grammar::VT_MARK),
            ("nonmutualand", grammar::VT_NONMUTUALAND),
            ("nonmutualor", grammar::VT_NONMUTUALOR),
            ("nonrepeated", grammar::VT_NONREPEATED),
            ("not", grammar::VT_NOT),
            ("note", grammar::VT_NOTE),
            ("now", grammar::VT_NOW),
            ("occurrence", grammar::VT_OCCURRENCE),
            ("onlyoninput", grammar::VT_ONLYONINPUT),
            ("onlyonoutput", grammar::VT_ONLYONOUTPUT),
            ("onlyother", grammar::VT_ONLYOTHER),
            ("or", grammar::VT_OR),
            ("other", grammar::VT_OTHER),
            ("otherselections", grammar::VT_OTHERSELECTIONS),
            ("picture", grammar::VT_PICTURE),
            ("prefix", grammar::VT_PREFIX),
            ("prescribedselections", grammar::VT_PRESCRIBEDSELECTIONS),
            ("proper", grammar::VT_PROPER),
            ("punctuation", grammar::VT_PUNCTUATION),
            ("ref", grammar::VT_REF),
            ("reference", grammar::VT_REFERENCE),
            ("relevance", grammar::VT_RELEVANCE),
            // ("repeat",  new FunctionToken("repeat")),
            ("repeatcontext", grammar::VT_REPEATCONTEXT),
            ("repeatcounter", grammar::VT_REPEATCOUNTER),
            ("select", grammar::VT_SELECT),
            ("selectionoptions", grammar::VT_SELECTIONOPTIONS),
            ("sensitive", grammar::VT_SENSITIVE),
            ("simplify", grammar::VT_SIMPLIFY),
            ("spanrelevance", grammar::VT_SPANRELEVANCE),
            ("style", grammar::VT_STYLE),
            ("sure", grammar::VT_SURE),
            ("template", grammar::VT_TEMPLATE),
            ("templaterelevance", grammar::VT_TEMPLATERELEVANCE),
            ("textfile", grammar::VT_TEXTFILE),
            ("then", grammar::VT_THEN),
            ("to", grammar::VT_TO),
            ("today", grammar::VT_TODAY),
            ("true", grammar::VT_TRUE),
            // ("unrepeated", new FunctionToken("unrepeated")),
            ("upper", grammar::VT_UPPER),
            ("using", grammar::VT_USING),
            ("value", grammar::VT_VALUE),
            ("where", grammar::VT_WHERE),
            ("with", grammar::VT_WITH),
            ("xor", grammar::VT_XOR),
            ]);

        CeScanner {
            reserved_words
        }
    }

    fn next_token(&self, env : &mut ScannerEnv, symbol_table: &HashMap<String, Box<dyn Token>>, variables : &HashMap<String, Variable>) -> Box<dyn Token> {
        let mut lexema: String = String::new();
        let mut state = 0;
        let mut ch: char;

        loop {
            match state {
                0 => {
                    ch = env.next_char();

                    if ch.is_whitespace() {
                        state = 0;
                        continue;
                    }
                    if ch.is_alphabetic() {
                        /*
                         * If the name of your variable includes a space or any character 
                         * outside of the letter characters from the Latin-1 Supplement or 
                         * Latin Extended-A Unicode character set, then the Dictionary 
                         * Editor will automatically add single quotes around the variable 
                         * when the variable is added into a field in the template.
                         */
                        lexema.push(ch);
                        state = 1;
                        continue;
                    }
                    if ch.is_digit(10) {
                        lexema.push(ch);
                        state = 40;
                        continue;
                    }
                    if ch == '\'' {
                        /* Quoting Variable Names
                         * --------------------------
                         * Remember that Variable names containing certain characters (including spaces) 
                         * will need to be quoted using single quotes:
                         * 'Tax Regulations Apply'
                         * For a list of characters that do not need quoting see the Characters used in 
                         * Identifier names topic in the online documentation. 
                         */
                        lexema.push(ch);
                        state = 3;
                        continue;
                    }
                    if ch == '\"' {
                        /* Single vs Double quotes
                         * --------------------------
                         * Note that the value from text selection variables are double-quoted in 
                         * the business rule. If the text selection variable also had spaces or 
                         * other characters that need to be quoted in identifiers, the business 
                         * rule would look like this:
                         * ['Contract Region' Is "North America" This Agreement is subject to the 
                         * laws of the state of New York
                         * ]
                         */
                        state = 10;
                        continue;
                    }
                    if ch == '+' || ch == '-' {
                        lexema.push(ch);
                        state = 20;
                        continue;
                    }
                    if ch == '*' {
                        lexema.push(ch);
                        state = 21;
                        continue;
                    }
                    if ch == '/' {
                        lexema.push(ch);
                        state = 22;
                        continue;
                    }
                    if ch == '(' {
                        state = 30;
                        continue;
                    }
                    if ch == ')' {
                        state = 31;
                        continue;
                    }
                    if ch == ',' {
                        state = 32;
                        continue;
                    }
                    if ch == '#' {
                        state = 100;
                        continue;
                    }
                    // TEXT
                    lexema.push(ch);
                    state = 999;
                    continue;
                },
                1 => { // Identifiers and reserved words
                    ch = env.next_char();

                    if ch.is_alphanumeric() {
                        lexema.push(ch);
                        state = 1;
                        continue;
                    }
                    state = 2;
                    continue;
                },
                2 => {
                    env.retract();
                    /* Reserved words.
                    * ---------------------------------------
                    * There are a number of reserved words which variable names cannot contain 
                    * such as AND and NOT. The full list is as follows:
                    * alt, and, answered, as, attach, attribute, authornote, by, capitals, cell, 
                    * cloaked, collect, collectvalues, committed, datatype, deferred, definite, doctitle, 
                    * document, else, every, exists, export, expressiontext, false, foreach, format, 
                    * from, hyperlink, if, ifknownelse, include, is, isatleast, isatmost, islessthan, 
                    * ismorethan, isnot, known, knowntrue, label, list, lower, mark, nonmutualand, 
                    * nonmutualor, nonrepeated, not, note, now, occurrence, onlyoninput, onlyonoutput, 
                    * onlyother, or, other, otherselections, picture, prefix, prescribedselections,
                    * proper, punctuation, ref, reference, relevance, repeat, repeatcontext, 
                    * repeatcounter, select, selectionoptions, sensitive, simplify, spanrelevance, 
                    * style, sure, template, templaterelevance, textfile, then, to, today, true, 
                    * unrepeated, upper, using, value, where, with, xor 
                    */

                    lexema = lexema.to_lowercase();

                    if symbol_table.contains_key(lexema.as_str()) {
                        panic!("Token already exists");
                        //let token = symbol_table.get(lexema.as_str()).unwrap();
                        // return Box::new(*token.clone());
                    }

                    if self.reserved_words.contains_key(lexema.as_str()) {
                        let tag = self.reserved_words.get(lexema.as_str()).unwrap();
                        return match lexema.as_str() {
                            "is" => Box::new(RelOpToken::create(*tag, RelOpType::Is)),
                            "isatleast" => Box::new(RelOpToken::create(*tag, RelOpType::IsAtLeast)),
                            "isatmost" => Box::new(RelOpToken::create(*tag, RelOpType::IsAtMost)),
                            "islessthan" => Box::new(RelOpToken::create(*tag, RelOpType::IsLessThan)),
                            "ismorethan" => Box::new(RelOpToken::create(*tag, RelOpType::IsMoreThan)),
                            _ => Box::new(SimpleToken::from_tag(*tag)),
                        };
                    }

                    {
                         let tag = self.reserved_words.get(lexema.as_str());

                         match tag {
                            Some(t) => {
                                // if *t == grammar::VT_RELOP {
                                //     match lexema.as_str() {
                                //         "is" => return Box::new(RelOpToken::create(&grammar::VT_RELOP, RelOpType::Is, )),
                                //         "isatleast" => return Box::new(RelOpToken::create(&grammar::VT_RELOP, RelOpType::IsAtLeast, )),
                                //         "isatmost" => return Box::new(RelOpToken::create(&grammar::VT_RELOP, RelOpType::IsAtMost, )),
                                //         "islessthan" => return Box::new(RelOpToken::create(&grammar::VT_RELOP, RelOpType::IsLessThan, )),
                                //         "ismorethan" => return Box::new(RelOpToken::create(&grammar::VT_RELOP, RelOpType::IsMoreThan, )),
                                //         _ => {}
                                //     }
                                //     // return Box::new(ValuedToken::create(RelOpType::LessThan, ));
                                // }
                                if *t == grammar::VT_ADDOP {
                                    return match lexema.as_str() {
                                        // "and" => return Box::new(RelOpToken::create(&grammar::VT_ADDOP, RelOpType::And, )),
                                        // "or" => return Box::new(RelOpToken::create(&grammar::VT_ADDOP, RelOpType::Or, )),
                                        _ => Box::new(UnknowToken::create(grammar::VT_UNKNOW, lexema)),
                                    }
                                    // return Box::new(AddOpToken::create(lexema));
                                }
                                if *t == grammar::VT_MULOP {
                                    return match lexema.as_str() {
                                        // "not" => return Box::new(RelOpToken::create(&grammar::VT_MULOP, RelOpType::Not, )),
                                        _ => Box::new(UnknowToken::create(grammar::VT_UNKNOW, lexema)),
                                    }
                                    // return Box::new(MulOpToken::create(lexema));
                                }
                            },
                            _ => {},
                         }
                    }

                    if variables.contains_key(lexema.as_str()) {
                        return Box::new(VariableToken::create(grammar::VT_VARIABLE, lexema));
                    }
                    else {
                        return Box::new(FunctionToken::create(grammar::VT_FUNCTION, lexema));
                    }
                },
                3 => {
                    /* Escape character and reserved words.
                    * ---------------------------------------
                    * If you include a single quote as part of the variable Name, the Variable 
                    * Editor will automatically add a backslash character to escape the quote: \'
                    * Similarly, if the name includes a backslash, the variable editor will 
                    * prefix this with a further backslash character \\
                    * If you add fields into the template as you type, you must remember to 
                    * escape quotes and backslashes where necessary.
                    * Additionally, you cannot use the following symbols as the variable name (but 
                    * they can be included as part of the variable name if quoted):
                    * + - * / // , ( ) # ? .
                    */
                    ch = env.next_char();

                    if ch.is_alphanumeric() {
                        lexema.push(ch);
                        state = 4;
                        continue;
                    }
                    if ch.is_whitespace() {
                        lexema.push(ch);
                        state = 3;
                        continue;
                    }
                    if "+-*/,()#?.".contains(ch) { // caracter //?
                        lexema.push(ch);
                        state = 4;
                        continue;
                    }
                    if ch == '\\' {
                        state = 4;
                        continue;
                    }
                    state = 999;
                    continue;
                },
                4 => {
                    /* Escape character.
                    * ---------------------------------------
                    * If you include a single quote as part of the variable Name, the Variable 
                    * Editor will automatically add a backslash character to escape the quote: \'
                    * Similarly, if the name includes a backslash, the variable editor will 
                    * prefix this with a further backslash character \\
                    * If you add fields into the template as you type, you must remember to 
                    * escape quotes and backslashes where necessary.
                    * Additionally, you cannot use the following symbols as the variable name (but 
                    * they can be included as part of the variable name if quoted):
                    * + - * / // , ( ) # ? .
                    */
                    ch = env.next_char();

                    if ch.is_alphanumeric() {
                        lexema.push(ch);
                        state = 4;
                        continue;
                    }
                    if ch.is_whitespace() {
                        lexema.push(ch);
                        state = 4;
                        continue;
                    }
                    if false {
                        lexema.push(ch);
                        state = 4;
                        continue;
                    }
                    if "+-*/,()#?.".contains(ch) { // caracter //?
                        lexema.push(ch);
                        state = 4;
                        continue;
                    }
                    if ch == '\\' {
                        state = 5;
                        continue;
                    }
                    if ch == '\''{
                        state = 6;
                        continue;
                    }
                    state = 999;
                    continue;
                },
                5 => {
                    ch = env.next_char();

                    if ch == '\\' {
                        lexema.push(ch);
                        state = 4;
                        continue;
                    }
                    state = 999;
                    continue;
                },
                6 => {
                    if variables.contains_key(lexema.as_str()) {
                        return Box::new(VariableToken::create(grammar::VT_VARIABLE, lexema));
                    }   
                    else {
                        return Box::new(UnknowToken::create(grammar::VT_UNKNOW, lexema));
                    }
                },
                9 => {
                    env.retract();
                    if variables.contains_key(lexema.as_str()) {
                         return Box::new(VariableToken::create(grammar::VT_VARIABLE, lexema));
                    }
                    else {
                        return Box::new(UnknowToken::create(grammar::VT_UNKNOW, lexema));
                    }
                },
                10 => {
                    ch = env.next_char();

                    if ch == '\"' {
                        state = 11;
                        continue;
                    }
                    if ch != '\n' {
                        lexema.push(ch);
                        state = 10;
                        continue;
                    }
                    state = 999;
                    continue;
                },
                11 => {
                    return Box::new(LiteralToken::create(grammar::VT_LITERAL, lexema));
                },
                20 => {
                    return match lexema.as_str() {
                        "+" => return Box::new(ValuedToken::create(grammar::VT_ADDOP, AddOpType::Plus)),
                        "-" => return Box::new(ValuedToken::create(grammar::VT_ADDOP, AddOpType::Minus)),
                        _ => Box::new(UnknowToken::create(grammar::VT_UNKNOW, lexema)),
                    };
                },
                21 => {
                    return match lexema.as_str() {
                        "*" => return Box::new(ValuedToken::create(grammar::VT_MULOP, MulOpType::Multiply)),
                        "/" => return Box::new(ValuedToken::create(grammar::VT_MULOP, MulOpType::Divide)),
                        _ => Box::new(UnknowToken::create(grammar::VT_UNKNOW, lexema)),
                    };
                },
                22 => {
                    ch = env.next_char();

                    if ch == '/' {
                        lexema = ch.to_string();
                        state = 23;
                        continue;
                    }
                    state = 24;
                    continue;
                },
                23 => {
                    return Box::new(MulOpToken::create(grammar::VT_MULOP, MulOpType::Divide));
                },
                24 => {
                    env.retract();
                    return Box::new(MulOpToken::create(grammar::VT_MULOP, MulOpType::Divide));
                },
                30 => {
                    return Box::new(SimpleToken::from_tag(grammar::VT_LPAR));
                },
                31 => {
                    return Box::new(SimpleToken::from_tag(grammar::VT_RPAR));
                },
                32 => {
                    return Box::new(SimpleToken::from_tag(grammar::VT_COMMA));
                },
                40 => {
                    ch = env.next_char();

                    if ch.is_digit(10) {
                        lexema = ch.to_string();
                        state = 40;
                        continue;
                    }
                    if ch == '.'  {
                        lexema = ch.to_string();
                        state = 41;
                        continue;
                    }
                    state = 45;
                    continue;
                },
                41 => {
                    ch = env.next_char();

                    if ch.is_digit(10) {
                        lexema = ch.to_string();
                        state = 42;
                        continue;
                    }
                    state = 999;
                    continue;
                },
                42 => {
                    ch = env.next_char();

                    if ch.is_digit(10) {
                        lexema = ch.to_string();
                        state = 42;
                        continue;
                    }
                    state = 49;
                    continue;
                },
                45 => {
                    env.retract();
                    return Box::new(IntegerToken::create(grammar::VT_INTEGER, lexema.parse::<isize>().unwrap()));
                },
                49 => {
                    env.retract();
                    return Box::new(DecimalToken::create(grammar::VT_DECIMAL, lexema.parse::<f64>().unwrap()));
                },
                100 => {
                    return Box::new(SimpleToken::from_tag(grammar::VT_ENDMARK));
                },
                999 => {
                    return Box::new(UnknowToken::create(grammar::VT_UNKNOW, lexema));
                }

                _ => {
                    panic!("Error: Invalid state {}", state);
                }
            }
        }
    }
}

