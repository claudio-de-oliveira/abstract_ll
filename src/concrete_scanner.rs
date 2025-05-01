use std::collections::HashMap;

use crate::{abstract_parser::Scanner, token::Token, scanner_environment::ScannerEnv};

pub struct ConcreteScanner {
    reserved_words: HashMap<String, Token>, // Maps reserved words to their token types
}

impl Scanner for ConcreteScanner {
    //  Dictionary<string, Token> reservedWords = new()
    // {
    //     { "alt",  Token.ALT },
    //     { "and",  Token.AND },
    //     { "answered",  Token.ANSWERED },
    //     { "as",  Token.AS },
    //     { "attach",  Token.ATTACH },
    //     { "attribute",  Token.ATTRIBUTE },
    //     { "authornote",  Token.AUTHORNOTE },
    //     { "by",  Token.BY },
    //     { "capitals",  Token.CAPITALS },
    //     { "cell",  Token.CELL },
    //     { "cloaked", Token.CLOAKED },
    //     { "collect",  Token.COLLECT },
    //     { "collectvalues",  Token.COLLECTVALUES },
    //     { "committed",  Token.COMMITTED },
    //     { "datatype",  Token.DATATYPE },
    //     { "deferred",  Token.DEFERRED },
    //     { "definite",  Token.DEFINITE },
    //     { "doctitle",  Token.DOCTITLE },
    //     { "document", Token.DOCUMENT },
    //     { "else",  Token.ELSE },
    //     { "every",  Token.EVERY },
    //     { "exists",  Token.EXISTS },
    //     { "export",  Token.EXPORT },
    //     { "expressiontext",  Token.EXPRESSIONTEXT },
    //     { "false",  Token.FALSE },
    //     { "foreach",  Token.FOREACH },
    //     { "format",  Token.FORMAT },
    //     { "from",  Token.FROM },
    //     { "hyperlink",  Token.HYPERLINK },
    //     { "if", Token.IF },                           // test that a value equals another value
    //     { "ifknownelse",  Token.IFKNOWNELSE },
    //     { "include",  Token.INCLUDE },
    //     { "is",  Token.RELOP },                       // test that a value equals another value
    //     { "isatleast",  Token.RELOP},                 // test that a value is more than or equal to another value
    //     { "isatmost",  Token.RELOP },                 // test that a value is less than or equal to another value
    //     { "islessthan",  Token.RELOP },               // test that a value is less than another value
    //     { "ismorethan",  Token.RELOP },               // test that a value is more than another value
    //     { "isnot",  Token.RELOP },                    // test that a value is not equal to another value
    //     { "known", Token.KNOWN },
    //     { "knowntrue",  Token.KNOWNTRUE },
    //     { "label",  Token.LABEL },
    //     { "list",  Token.LIST },
    //     { "lower",  Token.LOWER },
    //     { "mark",  Token.MARK },
    //     { "nonmutualand",  Token.NONMUTUALAND },
    //     { "nonmutualor",  Token.NONMUTUALOR },
    //     { "nonrepeated",  Token.NONREPEATED },
    //     { "not", Token.NOT },
    //     { "note",  Token.NOTE },
    //     { "now",  Token.NOW },
    //     { "occurrence",  Token.OCCURRENCE },
    //     { "onlyoninput",  Token.ONLYONINPUT },
    //     { "onlyonoutput",  Token.ONLYONOUTPUT },
    //     { "onlyother",  Token.ONLYOTHER },
    //     { "or",  Token.OR },
    //     { "other", Token.OTHER },
    //     { "otherselections",  Token.OTHERSELECTIONS },
    //     { "picture",  Token.PICTURE },
    //     { "prefix",  Token.PREFIX },
    //     { "prescribedselections",  Token.PRESCRIBEDSELECTIONS },
    //     { "proper",  Token.PROPER },
    //     { "punctuation",  Token.PUNCTUATION },
    //     { "ref", Token.REF },
    //     { "reference",  Token.REFERENCE },
    //     { "relevance",  Token.RELEVANCE },
    //     { "repeat",  new FunctionToken("repeat") },
    //     { "repeatcontext",  Token.REPEATCONTEXT },
    //     { "repeatcounter",  Token.REPEATCOUNTER },
    //     { "select", Token.SELECT },
    //     { "selectionoptions",  Token.SELECTIONOPTIONS },
    //     { "sensitive",  Token.SENSITIVE },
    //     { "simplify",  Token.SIMPLIFY },
    //     { "spanrelevance",  Token.SPANRELEVANCE },
    //     { "style",  Token.STYLE },
    //     { "sure",  Token.SURE },
    //     { "template", Token.TEMPLATE },
    //     { "templaterelevance",  Token.TEMPLATERELEVANCE },
    //     { "textfile",  Token.TEXTFILE },
    //     { "then",  Token.THEN },
    //     { "to",  Token.TO },
    //     { "today",  Token.TODAY },
    //     { "true",  Token.TRUE },
    //     { "unrepeated", new FunctionToken("unrepeated") },
    //     { "upper",  Token.UPPER },
    //     { "using",  Token.USING },
    //     { "value", Token.VALUE },
    //     { "where",  Token.WHERE },
    //     { "with", Token.WITH },
    //     { "xor", Token.XOR },
    // };

    fn next_token(&self, _env: &mut ScannerEnv) -> Token {
        // Implementation of the next_token method
        // This is where you would implement the logic to return the next token from the input stream
        // For now, we will just return a placeholder token
        // Token::new(RELOP) // Placeholder implementation
        Token::new(self.reserved_words.get("alt").unwrap().get_tag().clone())
    }
    
    fn initialize(&mut self) {
        // Initialize the scanner with the environment
    }
}