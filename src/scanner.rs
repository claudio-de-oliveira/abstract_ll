use std::default;
use std::{fmt, ptr::eq};
use std::collections::HashMap;

use crate::{ce_tag, ce_token, scanner_environment::ScannerEnv, tag::Tag, token::Token};

pub trait Scanner {
    fn new() -> Self;
    fn next_token(&self, env : &mut ScannerEnv) -> Token;
}


#[derive(Debug)]
pub struct CeScanner {
    pub reserved_words: HashMap<&'static str, Token>
}

impl fmt::Display for CeScanner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self.reserved_words)
    }
}

impl Scanner for CeScanner {

    fn new() -> Self {
        let reserved_words = HashMap::from([
            ("alt", (*ce_token::ALT).clone()),
            ("and", (*ce_token::AND).clone()),
            ("answered", (*ce_token::ANSWERED).clone()),
            ("as", (*ce_token::AS).clone()),
            ("attach", (*ce_token::ATTACH).clone()),
            ("attribute", (*ce_token::ATTRIBUTE).clone()),
            ("authornote", (*ce_token::AUTHORNOTE).clone()),
            ("by", (*ce_token::BY).clone()),
            ("capitals", (*ce_token::CAPITALS).clone()),
            ("cell", (*ce_token::CELL).clone()),
            ("cloaked", (*ce_token::CLOAKED).clone()),
            ("collect", (*ce_token::COLLECT).clone()),
            ("collectvalues", (*ce_token::COLLECTVALUES).clone()),
            ("committed", (*ce_token::COMMITTED).clone()),
            ("datatype", (*ce_token::DATATYPE).clone()),
            ("deferred", (*ce_token::DEFERRED).clone()),
            ("definite", (*ce_token::DEFINITE).clone()),
            ("doctitle", (*ce_token::DOCTITLE).clone()),
            ("document", (*ce_token::DOCUMENT).clone()),
            ("else", (*ce_token::ELSE).clone()),
            ("every", (*ce_token::EVERY).clone()),
            ("exists", (*ce_token::EXISTS).clone()),
            ("export", (*ce_token::EXPORT).clone()),
            ("expressiontext", (*ce_token::EXPRESSIONTEXT).clone()),
            ("false", (*ce_token::FALSE).clone()),
            ("foreach", (*ce_token::FOREACH).clone()),
            ("format", (*ce_token::FORMAT).clone()),
            ("from", (*ce_token::FROM).clone()),
            ("hyperlink", (*ce_token::HYPERLINK).clone()),
            ("if", (*ce_token::IF).clone()),                           // test that a value equals another value
            ("ifknownelse", (*ce_token::IFKNOWNELSE).clone()),
            ("include", (*ce_token::INCLUDE).clone()),
            ("is", (*ce_token::RELOP).clone()),                       // test that a value equals another value
            ("isatleast", (*ce_token::RELOP).clone()),                 // test that a value is more than or equal to another value
            ("isatmost", (*ce_token::RELOP).clone()),                 // test that a value is less than or equal to another value
            ("islessthan", (*ce_token::RELOP).clone()),               // test that a value is less than another value
            ("ismorethan", (*ce_token::RELOP).clone()),               // test that a value is more than another value
            ("isnot", (*ce_token::RELOP).clone()),                    // test that a value is not equal to another value
            ("known", (*ce_token::KNOWN).clone()),
            ("knowntrue", (*ce_token::KNOWNTRUE).clone()),
            ("label", (*ce_token::LABEL).clone()),
            ("list", (*ce_token::LIST).clone()),
            ("lower", (*ce_token::LOWER).clone()),
            ("mark", (*ce_token::MARK).clone()),
            ("nonmutualand", (*ce_token::NONMUTUALAND).clone()),
            ("nonmutualor", (*ce_token::NONMUTUALOR).clone()),
            ("nonrepeated", (*ce_token::NONREPEATED).clone()),
            ("not", (*ce_token::NOT).clone()),
            ("note", (*ce_token::NOTE).clone()),
            ("now", (*ce_token::NOW).clone()),
            ("occurrence", (*ce_token::OCCURRENCE).clone()),
            ("onlyoninput", (*ce_token::ONLYONINPUT).clone()),
            ("onlyonoutput", (*ce_token::ONLYONOUTPUT).clone()),
            ("onlyother", (*ce_token::ONLYOTHER).clone()),
            ("or", (*ce_token::OR).clone()),
            ("other", (*ce_token::OTHER).clone()),
            ("otherselections", (*ce_token::OTHERSELECTIONS).clone()),
            ("picture", (*ce_token::PICTURE).clone()),
            ("prefix", (*ce_token::PREFIX).clone()),
            ("prescribedselections", (*ce_token::PRESCRIBEDSELECTIONS).clone()),
            ("proper", (*ce_token::PROPER).clone()),
            ("punctuation", (*ce_token::PUNCTUATION).clone()),
            ("ref", (*ce_token::REF).clone()),
            ("reference", (*ce_token::REFERENCE).clone()),
            ("relevance", (*ce_token::RELEVANCE).clone()),
            // ("repeat",  new FunctionToken("repeat")).clone()),
            ("repeatcontext", (*ce_token::REPEATCONTEXT).clone()),
            ("repeatcounter", (*ce_token::REPEATCOUNTER).clone()),
            ("select", (*ce_token::SELECT).clone()),
            ("selectionoptions", (*ce_token::SELECTIONOPTIONS).clone()),
            ("sensitive", (*ce_token::SENSITIVE).clone()),
            ("simplify", (*ce_token::SIMPLIFY).clone()),
            ("spanrelevance", (*ce_token::SPANRELEVANCE).clone()),
            ("style", (*ce_token::STYLE).clone()),
            ("sure", (*ce_token::SURE).clone()),
            ("template", (*ce_token::TEMPLATE).clone()),
            ("templaterelevance", (*ce_token::TEMPLATERELEVANCE).clone()),
            ("textfile", (*ce_token::TEXTFILE).clone()),
            ("then", (*ce_token::THEN).clone()),
            ("to", (*ce_token::TO).clone()),
            ("today", (*ce_token::TODAY).clone()),
            ("true", (*ce_token::TRUE).clone()),
            // ("unrepeated", new FunctionToken("unrepeated")).clone()),
            ("upper", (*ce_token::UPPER).clone()),
            ("using", (*ce_token::USING).clone()),
            ("value", (*ce_token::VALUE).clone()),
            ("where", (*ce_token::WHERE).clone()),
            ("with", (*ce_token::WITH).clone()),
            ("xor", (*ce_token::XOR).clone())
            ]);

        CeScanner {
            reserved_words
        }
    }

    fn next_token(&self, env : &mut ScannerEnv) -> Token {
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

                    // if (((Environment)environment).SymbolTable.ContainsKey(lexema.ToLower()))
                    //     return ((Environment)environment).SymbolTable[lexema.ToLower()];

                    if self.reserved_words.contains_key(lexema.as_str()) {
                        let token = self.reserved_words.get(lexema.as_str()).unwrap();
                        return token.clone();
                    }

                    {
                         let token = self.reserved_words.get(lexema.as_str()).unwrap();
                         if token.get_tag().tag == ce_tag::VT_RELOP.tag {
                             // return new RelOpToken(lexema);
                         }
                    //     if (token.GetTag() == Tag.ADDOP)
                    //         return new AddOpToken(lexema);
                    //     if (token.GetTag() == Tag.MULOP)
                    //         return new MulOpToken(lexema);
                    //     return token;
                    }

                    // if (((Environment)environment).Variables.ContainsKey(lexema))
                    //     return new VariableToken(lexema);
                    // else
                    //     return new FunctionToken(lexema);
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
                    // if (((Environment)environment).Variables.ContainsKey(lexema))
                    //         return new VariableToken(lexema);
                    //     else
                    //         return new UnknowToken(lexema);
                },
                9 => {
                    env.retract();
                    // if (((Environment)environment).Variables.ContainsKey(lexema))
                    //     return new VariableToken(lexema);
                    // else
                    //     return new UnknowToken(lexema);
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
                    // return new LiteralToken(lexema);
                },
                20 => {
                    // return new AddOpToken(lexema);
                },
                21 => {
                    // return new MulOpToken(lexema);
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
                    //     return new MulOpToken(lexema);
                },
                24 => {
                    env.retract();
                    // return new MulOpToken(lexema);
                },
                30 => {
                        return (*ce_token::LPAR).clone();
                },
                31 => {
                    return (*ce_token::RPAR).clone();
                },
                32 => {
                    return (*ce_token::COMMA).clone();
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
                    // return new IntegerToken(long.Parse(lexema));
                },
                49 => {
                    env.retract();
                    // return new DecimalToken(double.Parse(lexema));
                },
                100 => {
                    return (*ce_token::ENDMARK).clone();
                },
                999 => {
                }

                _ => {
                    panic!("Error: Invalid state {}", state);
                }
            }
        }
    }
}

// impl Default for CeScanner {
//     fn default() -> Self {
//         Self::new()
//     }
// }