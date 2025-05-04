use array2d::Array2D;

use crate::rule::Rule;
use crate::token::ValuedToken;
use crate::tag::Tag;

// T E R M I N A I S
pub static VT_RELOP: Tag = Tag { tag: 0, name: "relop", num_att: 0, };
pub static VT_ADDOP: Tag = Tag { tag: 1, name: "addop", num_att: 0, };
pub static VT_MULOP: Tag = Tag { tag: 2, name: "mulop", num_att: 0, };
pub static VT_LPAR: Tag = Tag { tag: 3, name: "(", num_att: 0, };
pub static VT_RPAR: Tag = Tag { tag: 4, name: ")", num_att: 0, };
pub static VT_COMMA: Tag = Tag { tag: 5, name: ",", num_att: 0, };
pub static VT_VARIABLE: Tag = Tag { tag: 6, name: "variable", num_att: 0, };
pub static VT_INTEGER: Tag = Tag { tag: 7, name: "integer", num_att: 0, };
pub static VT_DECIMAL: Tag = Tag { tag: 8, name: "decimal", num_att: 0, };
pub static VT_LITERAL: Tag = Tag { tag: 9, name: "literal", num_att: 0, };
pub static VT_FUNCTION: Tag = Tag { tag: 10, name: "fx", num_att: 0, };
pub static VT_IF: Tag = Tag { tag: 11, name: "if", num_att: 0, };
pub static VT_THEN: Tag = Tag { tag: 12, name: "then", num_att: 0, };
pub static VT_ELSE: Tag = Tag { tag: 13, name: "else", num_att: 0, };
pub static VT_TRUE: Tag = Tag { tag: 14, name: "true", num_att: 0, };
pub static VT_FALSE: Tag = Tag { tag: 15, name: "false", num_att: 0, };
// pub static VT_IS: Tag = Tag { tag: 16, name: "is", num_att: 0, };
// pub static VT_ISATLEAST: Tag = Tag { tag: 17, name: "isatleast", num_att: 0, };
// pub static VT_ISATMOST: Tag = Tag { tag: 18, name: "isatmost", num_att: 0, };
// pub static VT_ISLESSTHAN: Tag = Tag { tag: 19, name: "islessthan", num_att: 0, };
// pub static VT_ISMORETHAN: Tag = Tag { tag: 20, name: "ismorethan", num_att: 0, };
// pub static VT_ISNOT: Tag = Tag { tag: 21, name: "isnot", num_att: 0, };
pub static VT_AND: Tag = Tag { tag: 22, name: "and", num_att: 0, };
pub static VT_OR: Tag = Tag { tag: 23, name: "or", num_att: 0, };
pub static VT_XOR: Tag = Tag { tag: 24, name: "xor", num_att: 0, };
pub static VT_NOT: Tag = Tag { tag: 25, name: "not", num_att: 0, };
pub static VT_SELECT: Tag = Tag { tag: 26, name: "select", num_att: 0, };
pub static VT_FROM: Tag = Tag { tag: 27, name: "from", num_att: 0, };
pub static VT_FORMAT: Tag = Tag { tag: 28, name: "format", num_att: 0, };
pub static VT_ENDMARK: Tag = Tag { tag: 29, name: "#", num_att: 0, };

pub static NUMBEROFTERMINALS: Tag = Tag { tag: 30, name: "NUMBEROFTERMINALS", num_att: 0, };

// ABS, ACOS, ASCENDING, ASIN, ATAN,
pub static VT_ALT: Tag = Tag { tag: 100, name: "alt", num_att: 0, };
pub static VT_ANSWERED: Tag = Tag { tag: 102, name: "answered", num_att: 0, };
pub static VT_AS: Tag = Tag { tag: 103, name: "as", num_att: 0, };
pub static VT_ATTACH: Tag = Tag { tag: 104, name: "attach", num_att: 0, };
pub static VT_ATTRIBUTE: Tag = Tag { tag: 105, name: "attribute", num_att: 0, };
pub static VT_AUTHORNOTE: Tag = Tag { tag: 106, name: "authornote", num_att: 0, };
pub static VT_BY: Tag = Tag { tag: 107, name: "by", num_att: 0, };
pub static VT_CAPITALS: Tag = Tag { tag: 108, name: "capitals", num_att: 0, };
pub static VT_CELL: Tag = Tag { tag: 109, name: "cell", num_att: 0, };
pub static VT_CLOAKED: Tag = Tag { tag: 110, name: "cloaked", num_att: 0, };
pub static VT_COLLECT: Tag = Tag { tag: 111, name: "collect", num_att: 0, };
pub static VT_COLLECTVALUES: Tag = Tag { tag: 112, name: "collectvalues", num_att: 0, };
pub static VT_COMMITTED: Tag = Tag { tag: 113, name: "committed", num_att: 0, };
pub static VT_DATATYPE: Tag = Tag { tag: 114, name: "datatype", num_att: 0, };
pub static VT_DEFERRED: Tag = Tag { tag: 115, name: "deferred", num_att: 0, };
pub static VT_DEFINITE: Tag = Tag { tag: 116, name: "definite", num_att: 0, };
pub static VT_DOCTITLE: Tag = Tag { tag: 117, name: "doctitle", num_att: 0, };
pub static VT_DOCUMENT: Tag = Tag { tag: 118, name: "document", num_att: 0, };
pub static VT_EVERY: Tag = Tag { tag: 120, name: "every", num_att: 0, };
pub static VT_EXISTS: Tag = Tag { tag: 121, name: "exists", num_att: 0, };
pub static VT_EXPORT: Tag = Tag { tag: 122, name: "export", num_att: 0, };
pub static VT_EXPRESSIONTEXT: Tag = Tag { tag: 123, name: "expressiontext", num_att: 0, };
pub static VT_FOREACH: Tag = Tag { tag: 125, name: "foreach", num_att: 0, };
pub static VT_HYPERLINK: Tag = Tag { tag: 128, name: "hyperlink", num_att: 0, };
pub static VT_IFKNOWNELSE: Tag = Tag { tag: 130, name: "ifknownelse", num_att: 0, };
pub static VT_INCLUDE: Tag = Tag { tag: 131, name: "include", num_att: 0, };
pub static VT_KNOWN: Tag = Tag { tag: 138, name: "known", num_att: 0, };
pub static VT_KNOWNTRUE: Tag = Tag { tag: 139, name: "knowntrue", num_att: 0, };
pub static VT_LABEL: Tag = Tag { tag: 140, name: "label", num_att: 0, };
pub static VT_LIST: Tag = Tag { tag: 141, name: "list", num_att: 0, };
pub static VT_LOWER: Tag = Tag { tag: 142, name: "lower", num_att: 0, };
pub static VT_MARK: Tag = Tag { tag: 143, name: "mark", num_att: 0, };
pub static VT_NONMUTUALAND: Tag = Tag { tag: 144, name: "nonmutualand", num_att: 0, };
pub static VT_NONMUTUALOR: Tag = Tag { tag: 145, name: "nonmutualor", num_att: 0, };
pub static VT_NONREPEATED: Tag = Tag { tag: 146, name: "nonrepeated", num_att: 0, };
pub static VT_NOTE: Tag = Tag { tag: 148, name: "note", num_att: 0, };
pub static VT_NOW: Tag = Tag { tag: 149, name: "now", num_att: 0, };
pub static VT_OCCURRENCE: Tag = Tag { tag: 150, name: "occurrence", num_att: 0, };
pub static VT_ONLYONINPUT: Tag = Tag { tag: 151, name: "onlyoninput", num_att: 0, };
pub static VT_ONLYONOUTPUT: Tag = Tag { tag: 152, name: "onlyonoutput", num_att: 0, };
pub static VT_ONLYOTHER: Tag = Tag { tag: 153, name: "onlyother", num_att: 0, };
pub static VT_OTHER: Tag = Tag { tag: 155, name: "other", num_att: 0, };
pub static VT_OTHERSELECTIONS: Tag = Tag { tag: 156, name: "otherselections", num_att: 0, };
pub static VT_PICTURE: Tag = Tag { tag: 157, name: "picture", num_att: 0, };
pub static VT_PREFIX: Tag = Tag { tag: 158, name: "prefix", num_att: 0, };
pub static VT_PRESCRIBEDSELECTIONS: Tag = Tag { tag: 159, name: "prescribedselections", num_att: 0, };
pub static VT_PROPER: Tag = Tag { tag: 160, name: "proper", num_att: 0, };
pub static VT_PUNCTUATION: Tag = Tag { tag: 161, name: "punctuation", num_att: 0, };
pub static VT_REF: Tag = Tag { tag: 162, name: "ref", num_att: 0, };
pub static VT_REFERENCE: Tag = Tag { tag: 163, name: "reference", num_att: 0, };
pub static VT_RELEVANCE: Tag = Tag { tag: 164, name: "relevance", num_att: 0, };
pub static VT_REPEAT: Tag = Tag { tag: 165, name: "repeat", num_att: 0, };
pub static VT_REPEATCONTEXT: Tag = Tag { tag: 166, name: "repeatcontext", num_att: 0, };
pub static VT_REPEATCOUNTER: Tag = Tag { tag: 167, name: "repeatcounter", num_att: 0, };
pub static VT_SELECTIONOPTIONS: Tag = Tag { tag: 169, name: "selectionoptions", num_att: 0, };
pub static VT_SENSITIVE: Tag = Tag { tag: 170, name: "sensitive", num_att: 0, };
pub static VT_SIMPLIFY: Tag = Tag { tag: 171, name: "simplify", num_att: 0, };
pub static VT_SPANRELEVANCE: Tag = Tag { tag: 172, name: "spanrelevance", num_att: 0, };
pub static VT_STYLE: Tag = Tag { tag: 173, name: "style", num_att: 0, };
pub static VT_SURE: Tag = Tag { tag: 174, name: "sure", num_att: 0, };
pub static VT_TEMPLATE: Tag = Tag { tag: 175, name: "template", num_att: 0, };
pub static VT_TEMPLATERELEVANCE: Tag = Tag { tag: 176, name: "templaterelevance", num_att: 0, };
pub static VT_TEXTFILE: Tag = Tag { tag: 177, name: "textfile", num_att: 0, };
pub static VT_TO: Tag = Tag { tag: 179, name: "to", num_att: 0, };
pub static VT_TODAY: Tag = Tag { tag: 180, name: "today", num_att: 0, };
pub static VT_UNREPEATED: Tag = Tag { tag: 182, name: "unrepeated", num_att: 0, };
pub static VT_UPPER: Tag = Tag { tag: 183, name: "upper", num_att: 0, };
pub static VT_USING: Tag = Tag { tag: 184, name: "using", num_att: 0, };
pub static VT_VALUE: Tag = Tag { tag: 185, name: "value", num_att: 0, };
pub static VT_WHERE: Tag = Tag { tag: 186, name: "where", num_att: 0, };
pub static VT_WITH: Tag = Tag { tag: 187, name: "with", num_att: 0, };

pub static VT_UNKNOW: Tag = Tag { tag: 998, name: "unknow", num_att: 0, };
pub static VT_EMPTY: Tag = Tag { tag: 999, name: "empty", num_att: 0, };

// #region A Ç Õ E S   S E M Â N T I C A S
pub static AC_IF: Tag = Tag { tag: 0, name: "@If", num_att: 3, };
pub static AC_OR: Tag = Tag { tag: 1, name: "@Or", num_att: 2, };
pub static AC_AND: Tag = Tag { tag: 2, name: "@And", num_att: 2, };
pub static AC_NOT: Tag = Tag { tag: 3, name: "@Not", num_att: 1, };
pub static AC_ADDOP: Tag = Tag { tag: 4, name: "@AddOp", num_att: 0, };
pub static AC_REL: Tag = Tag { tag: 5, name: "@Rel", num_att: 3, };
pub static AC_ADD: Tag = Tag { tag: 6, name: "@Add", num_att: 3, };
pub static AC_MUL: Tag = Tag { tag: 7, name: "@Mul", num_att: 3, };
pub static AC_VARIABLE: Tag = Tag { tag: 8, name: "@Variable", num_att: 1, };
pub static AC_INTEGER: Tag = Tag { tag: 9, name: "@Integer", num_att: 0, };
pub static AC_DECIMAL: Tag = Tag { tag: 10, name: "@Decimal", num_att: 0, };
pub static AC_LITERAL: Tag = Tag { tag: 11, name: "@Literal", num_att: 0, };
pub static AC_SKIP: Tag = Tag { tag: 12, name: "@Skip", num_att: 1, };
pub static AC_CALL: Tag = Tag { tag: 13, name: "@Call", num_att: 2, };
pub static AC_EMPTYLIST: Tag = Tag { tag: 14, name: "@EmptyList", num_att: 2, };
pub static AC_CREATELIST: Tag = Tag { tag: 15, name: "@CreateList", num_att: 1, };
pub static AC_INSERTLIST: Tag = Tag { tag: 16, name: "@InsertList", num_att: 2, };
pub static AC_TRUE: Tag = Tag { tag: 17, name: "@True", num_att: 0, };
pub static AC_FALSE: Tag = Tag { tag: 18, name: "@False", num_att: 0, };
pub static AC_PARAMETER: Tag = Tag { tag: 19, name: "@Parameter", num_att: 1, };
pub static AC_FUNCTION: Tag = Tag { tag: 20, name: "@Function", num_att: 0, };
pub static AC_MULOP: Tag = Tag { tag: 21, name: "@MulOp", num_att: 0, };
pub static AC_RELOP: Tag = Tag { tag: 22, name: "@RelOp", num_att: 0, };
pub static AC_SELECT: Tag = Tag { tag: 23, name: "@Select", num_att: 1, };
pub static AC_NOPARAMETER: Tag = Tag { tag: 24, name: "@NoParameter", num_att: 0, };
pub static AC_FIRSTITEM: Tag = Tag { tag: 25, name: "@FirstItem", num_att: 1, };
pub static AC_INSERTITEM: Tag = Tag { tag: 26, name: "@InsertItem", num_att: 2, };
pub static AC_SELECTITEM: Tag = Tag { tag: 27, name: "@SelectItem", num_att: 2, };
pub static AC_THEN: Tag = Tag { tag: 28, name: "@Then", num_att: 1, };
pub static AC_ELSE: Tag = Tag { tag: 29, name: "@Else", num_att: 1, };
pub static AC_TEST: Tag = Tag { tag: 30, name: "@Test", num_att: 1, };
pub static AC_FORMAT: Tag = Tag { tag: 31, name: "@Format", num_att: 2, };
pub static AC_ECHO: Tag = Tag { tag: 999, name: "@Echo", num_att: 1, };
pub static AC_DONE: Tag = Tag { tag: 1000, name: "@Done", num_att: 1, };

// #region N Ã O   T E R M I N A I S
pub static VN_START: Tag = Tag { tag: 0, name: "Start", num_att: 0, };
pub static VN_EXP: Tag = Tag { tag: 1, name: "Exp", num_att: 0, };
pub static VN_DISJ: Tag = Tag { tag: 2, name: "Disj", num_att: 0, };
pub static VN_DISJ_: Tag = Tag { tag: 3, name: "Disj'", num_att: 1, };
pub static VN_CONJ: Tag = Tag { tag: 4, name: "Conj", num_att: 0, };
pub static VN_CONJ_: Tag = Tag { tag: 5, name: "Conj'", num_att: 1, };
pub static VN_NEG: Tag = Tag { tag: 6, name: "Neg", num_att: 0, };
pub static VN_REL: Tag = Tag { tag: 7, name: "Rel", num_att: 0, };
pub static VN_REL_: Tag = Tag { tag: 8, name: "Rel'", num_att: 1, };
pub static VN_ADD: Tag = Tag { tag: 9, name: "Add", num_att: 0, };
pub static VN_ADD_: Tag = Tag { tag: 10, name: "Add'", num_att: 1, };
pub static VN_MULTIPLY: Tag = Tag { tag: 11, name: "Multiply", num_att: 0, };
pub static VN_MULTIPLY_: Tag = Tag { tag: 12, name: "Multiply'", num_att: 1, };
pub static VN_FACTOR: Tag = Tag { tag: 13, name: "Factor", num_att: 0, };
pub static VN_LIST: Tag = Tag { tag: 15, name: "List", num_att: 0, };
pub static VN_LIST_: Tag = Tag { tag: 16, name: "List'", num_att: 1, };
pub static VN_FUNCTION_: Tag = Tag { tag: 17, name: "Function'", num_att: 1, };
pub static VN_SELECTIONS: Tag = Tag { tag: 18, name: "Selections", num_att: 0, };
pub static VN_SELECTIONS_: Tag = Tag { tag: 19, name: "Selections'", num_att: 1, };
pub static VN_SELITEM: Tag = Tag { tag: 20, name: "SelItem", num_att: 0, };
pub static VN_SELITEM_: Tag = Tag { tag: 21, name: "SelItem'", num_att: 1, };
pub static VN_FMTEXP: Tag = Tag { tag: 22, name: "FmtExp", num_att: 0, };
pub static VN_FMTEXP_: Tag = Tag { tag: 23, name: "FmtExp'", num_att: 1, };

pub static NUMBEROFVARIABLES: Tag = Tag { tag: 24, name: "NUMBEROFVARIABLES", num_att: 0, };

#[derive(Debug, Clone, PartialEq)]
pub enum AddOpType {
    Plus,
    Minus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MulOpType {
    Multiply,
    Divide,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RelOpType {
    Is,
    IsAtLeast,
    IsAtMost,
    IsLessThan,
    IsMoreThan,
}

pub type UnknowToken = ValuedToken<String>;
pub type AddOpToken = ValuedToken<AddOpType>;
pub type MulOpToken = ValuedToken<MulOpType>;
pub type RelOpToken = ValuedToken<RelOpType>;
pub type VariableToken<C> = ValuedToken<C>;
pub type FunctionToken<C> = ValuedToken<C>;
pub type LiteralToken = ValuedToken<String>;
pub type IntegerToken = ValuedToken<isize>;
pub type DecimalToken = ValuedToken<f64>;


pub struct Grammar {
    rules: Vec<Rule>, // Array2D storing Tag elements
    m: Array2D<i32>, // Array2D storing Tag elements
    end_mark: &'static Tag,
}

impl Grammar {
    fn create_rules() -> Vec<Rule> {
        vec![
            // 0. & <START> ::= <FMTEXP> @DONE "\#"
            Rule {
                lhs: &VN_START,
                rhs: vec![&VN_FMTEXP, &AC_DONE, &VT_ENDMARK],
                first: vec![&VT_SELECT, &VT_FUNCTION, &VT_VARIABLE, &VT_INTEGER, &VT_DECIMAL, &VT_LITERAL, &VT_TRUE, &VT_FALSE, &VT_LPAR, &VT_NOT, &VT_IF],
                follow: vec![&VT_ENDMARK]
            },
            // 1. & <FMTEXP> ::= <EXP> <FMTEXP'>
            Rule {
                lhs: &VN_FMTEXP,
                rhs: vec![&VN_EXP, &VN_FMTEXP_],
                first: vec![&VT_SELECT, &VT_FUNCTION, &VT_VARIABLE, &VT_INTEGER, &VT_DECIMAL, &VT_LITERAL, &VT_TRUE, &VT_FALSE, &VT_LPAR, &VT_NOT, &VT_IF],
                follow: vec![&VT_THEN, &VT_ELSE, &VT_RPAR, &VT_COMMA, &VT_FORMAT, &VT_ENDMARK]
            },
            // 2. & <EXP> ::= "IF" <EXP> @TEST "THEN" <EXP> @THEN "ELSE" <EXP> @ELSE @IF
            Rule {
                lhs: &VN_EXP,
                rhs: vec![&VT_IF, &VN_EXP, &AC_TEST, &VT_THEN, &VN_EXP, &AC_THEN, &VT_ELSE, &VN_EXP, &AC_ELSE, &AC_IF],
                first: vec![&VT_IF],
                follow: vec![&VT_THEN, &VT_ELSE, &VT_RPAR, &VT_COMMA, &VT_FORMAT, &VT_ENDMARK]
            },
            // 3. & <EXP> ::= <DISJ>
            Rule {
                lhs: &VN_EXP,
                rhs: vec![&VN_DISJ],
                first: vec![&VT_SELECT, &VT_FUNCTION, &VT_VARIABLE, &VT_INTEGER, &VT_DECIMAL, &VT_LITERAL, &VT_TRUE, &VT_FALSE, &VT_LPAR, &VT_NOT],
                follow: vec![&VT_THEN, &VT_ELSE, &VT_RPAR, &VT_COMMA, &VT_FORMAT, &VT_ENDMARK],
            },
            // 4. & <DISJ> ::= <CONJ> <DISJ'>
            Rule {
                lhs: &VN_DISJ,
                rhs: vec![&VN_CONJ, &VN_DISJ_],
                first: vec![&VT_SELECT, &VT_FUNCTION, &VT_VARIABLE, &VT_INTEGER, &VT_DECIMAL, &VT_LITERAL, &VT_TRUE, &VT_FALSE, &VT_LPAR, &VT_NOT],
                follow: vec![&VT_THEN, &VT_ELSE, &VT_RPAR, &VT_COMMA, &VT_FORMAT, &VT_ENDMARK],
            },
            // 5. & <DISJ'> ::= "OR" <CONJ> @OR <DISJ'>
            Rule {
                lhs: &VN_DISJ_,
                rhs: vec![&VT_OR, &VN_CONJ, &AC_OR, &VN_DISJ_],
                first: vec![&VT_OR],
                follow: vec![&VT_THEN, &VT_ELSE, &VT_RPAR, &VT_COMMA, &VT_FORMAT, &VT_ENDMARK],
            },
            // 6. & <DISJ'> ::= @ECHO
            Rule {
                lhs: &VN_DISJ_,
                rhs: vec![&AC_ECHO],
                first: vec![&VT_THEN, &VT_ELSE, &VT_RPAR, &VT_COMMA, &VT_FORMAT, &VT_ENDMARK],
                follow: vec![],
            },
            // 7. & <CONJ> ::= <NEG> <CONJ'>
            Rule {
                lhs: &VN_CONJ,
                rhs: vec![&VN_NEG, &VN_CONJ_],
                first: vec![&VT_SELECT, &VT_FUNCTION, &VT_VARIABLE, &VT_INTEGER, &VT_DECIMAL, &VT_LITERAL, &VT_TRUE, &VT_FALSE, &VT_LPAR, &VT_NOT],
                follow: vec![&VT_OR, &VT_THEN, &VT_ELSE, &VT_RPAR, &VT_COMMA, &VT_FORMAT, &VT_ENDMARK],
            },
            // 8. & <CONJ'> ::= "AND" <NEG> @AND <CONJ'>
            Rule {
                lhs: &VN_CONJ_,
                rhs: vec![&VT_AND, &VN_NEG, &AC_AND, &VN_CONJ_],
                first: vec![&VT_AND],
                follow: vec![&VT_OR, &VT_THEN, &VT_ELSE, &VT_RPAR, &VT_COMMA, &VT_FORMAT, &VT_ENDMARK],
            },
            // 9. & <CONJ'> ::= @ECHO
            Rule {
                lhs: &VN_CONJ_,
                rhs: vec![&AC_ECHO],
                first: vec![&VT_RPAR, &VT_THEN, &VT_ELSE, &VT_OR, &VT_COMMA, &VT_FORMAT, &VT_ENDMARK],
                follow: vec![],
            },
            // 10. & <NEG> ::= "NOT" <NEG> @NOT
            Rule {
                lhs: &VN_NEG,
                rhs: vec![&VT_NOT, &VN_NEG, &AC_NOT],
                first: vec![&VT_NOT],
                follow: vec![&VT_AND, &VT_OR, &VT_THEN, &VT_ELSE, &VT_RPAR, &VT_COMMA, &VT_FORMAT, &VT_ENDMARK],
            },
            // 11. & <NEG> ::= <REL>
            Rule {
                lhs: &VN_NEG,
                rhs: vec![&VN_REL],
                first: vec![&VT_SELECT, &VT_FUNCTION, &VT_VARIABLE, &VT_INTEGER, &VT_DECIMAL, &VT_LITERAL, &VT_TRUE, &VT_FALSE, &VT_LPAR],
                follow: vec![&VT_AND, &VT_OR, &VT_THEN, &VT_ELSE, &VT_RPAR, &VT_COMMA, &VT_FORMAT, &VT_ENDMARK],
            },
            // 12. & <REL> ::= <ADD> <REL'>
            Rule {
                lhs: &VN_REL,
                rhs: vec![&VN_ADD, &VN_REL_],
                first: vec![&VT_SELECT, &VT_FUNCTION, &VT_VARIABLE, &VT_INTEGER, &VT_DECIMAL, &VT_LITERAL, &VT_TRUE, &VT_FALSE, &VT_LPAR],
                follow: vec![&VT_AND, &VT_OR, &VT_THEN, &VT_ELSE, &VT_RPAR, &VT_COMMA, &VT_FORMAT, &VT_ENDMARK],
            },
            // 13. & <REL'> ::= "RELOP" @RELOP <ADD> @REL
            Rule {
                lhs: &VN_REL_,
                rhs: vec![&VT_RELOP, &AC_RELOP, &VN_ADD, &AC_REL],
                first: vec![&VT_RELOP],
                follow: vec![&VT_AND, &VT_OR, &VT_THEN, &VT_ELSE, &VT_RPAR, &VT_COMMA, &VT_FORMAT, &VT_ENDMARK],
            },
            // 14. & <REL'> ::= @ECHO
            Rule {
                lhs: &VN_REL_,
                rhs: vec![&AC_ECHO],
                first: vec![&VT_RPAR, &VT_THEN, &VT_ELSE, &VT_OR, &VT_AND, &VT_COMMA, &VT_FORMAT, &VT_ENDMARK],
                follow: vec![],
            },
            // 15. & <ADD> ::= <MULTIPLY> <ADD'>
            Rule {
                lhs: &VN_ADD,
                rhs: vec![&VN_MULTIPLY, &VN_ADD_],
                first: vec![&VT_SELECT, &VT_FUNCTION, &VT_VARIABLE, &VT_INTEGER, &VT_DECIMAL, &VT_LITERAL, &VT_TRUE, &VT_FALSE, &VT_LPAR],
                follow: vec![&VT_AND, &VT_OR, &VT_THEN, &VT_ELSE, &VT_RPAR, &VT_COMMA, &VT_FORMAT, &VT_ENDMARK],
            },
            // 16. & <ADD'> ::= "ADDOP" @ADDOP <MULTIPLY> @ADD <ADD'>
            Rule {
                lhs: &VN_ADD_,
                rhs: vec![&VT_ADDOP, &AC_ADDOP, &VN_MULTIPLY, &AC_ADD, &VN_ADD_],
                first: vec![&VT_ADDOP],
                follow: vec![&VT_AND, &VT_OR, &VT_THEN, &VT_ELSE, &VT_RPAR, &VT_COMMA, &VT_FORMAT, &VT_ENDMARK],
            },
            // 17. & <ADD'> ::= @ECHO
            Rule {
                lhs: &VN_ADD_,
                rhs: vec![&AC_ECHO],
                first: vec![&VT_RPAR, &VT_THEN, &VT_ELSE, &VT_OR, &VT_AND, &VT_RELOP, &VT_COMMA, &VT_FORMAT, &VT_ENDMARK],
                follow: vec![],
            },
            // 18. & <MULTIPLY> ::= <F&AC_TOR> <MULTIPLY'>
            Rule {
                lhs: &VN_MULTIPLY,
                rhs: vec![&VN_FACTOR, &VN_MULTIPLY_],
                first: vec![&VT_SELECT, &VT_FUNCTION, &VT_VARIABLE, &VT_INTEGER, &VT_DECIMAL, &VT_LITERAL, &VT_TRUE, &VT_FALSE, &VT_LPAR],
                follow: vec![&VT_ADDOP, &VT_AND, &VT_OR, &VT_THEN, &VT_ELSE, &VT_RPAR, &VT_COMMA, &VT_FORMAT, &VT_ENDMARK],
            },
            // 19. & <MULTIPLY'> ::= "MULOP" @MULOP <F&AC_TOR> @MUL <MULTIPLY'>
            Rule {
                lhs: &VN_MULTIPLY_,
                rhs: vec![&VT_MULOP, &AC_MULOP, &VN_FACTOR, &AC_MUL, &VN_MULTIPLY_],
                first: vec![&VT_MULOP],
                follow: vec![&VT_ADDOP, &VT_AND, &VT_OR, &VT_THEN, &VT_ELSE, &VT_RPAR, &VT_COMMA, &VT_FORMAT, &VT_ENDMARK],
            },
            // 20. & <MULTIPLY'> ::= @ECHO
            Rule {
                lhs: &VN_MULTIPLY_,
                rhs: vec![&AC_ECHO],
                first: vec![&VT_RPAR, &VT_THEN, &VT_ELSE, &VT_OR, &VT_AND, &VT_RELOP, &VT_COMMA, &VT_ADDOP, &VT_FORMAT, &VT_ENDMARK ],
                follow: vec![],
            },
            // 21. & <F&AC_TOR> ::= "FUNCTION" @FUNCTION <FUNCTION'>
            Rule {
                lhs: &VN_FACTOR,
                rhs: vec![&VT_FUNCTION, &AC_FUNCTION, &VN_FUNCTION_],
                first: vec![&VT_FUNCTION],
                follow: vec![&VT_MULOP, &VT_ADDOP, &VT_AND, &VT_OR, &VT_THEN, &VT_ELSE, &VT_RPAR, &VT_COMMA, &VT_FORMAT, &VT_ENDMARK],
            },
            // 22. & <F&AC_TOR> ::= "VARIABLE" @VARIABLE
            Rule {
                lhs: &VN_FACTOR,
                rhs: vec![&VT_VARIABLE, &AC_VARIABLE],
                first: vec![&VT_VARIABLE],
                follow: vec![&VT_MULOP, &VT_ADDOP, &VT_AND, &VT_OR, &VT_THEN, &VT_ELSE, &VT_RPAR, &VT_COMMA, &VT_FORMAT, &VT_ENDMARK],
            },
            // 23. & <F&AC_TOR> ::= "INTEGER" @INTEGER
            Rule {
                lhs: &VN_FACTOR,
                rhs: vec![&VT_INTEGER, &AC_INTEGER],
                first: vec![&VT_INTEGER],
                follow: vec![&VT_MULOP, &VT_ADDOP, &VT_AND, &VT_OR, &VT_THEN, &VT_ELSE, &VT_RPAR, &VT_COMMA, &VT_FORMAT, &VT_ENDMARK],
            },
            // 24. & <F&AC_TOR> ::= "DECIMAL" @DECIMAL
            Rule {
                lhs: &VN_FACTOR,
                rhs: vec![&VT_DECIMAL, &AC_DECIMAL],
                first: vec![&VT_DECIMAL],
                follow: vec![&VT_MULOP, &VT_ADDOP, &VT_AND, &VT_OR, &VT_THEN, &VT_ELSE, &VT_RPAR, &VT_COMMA, &VT_FORMAT, &VT_ENDMARK],
            },
            // 25. & <F&AC_TOR> ::= "LITERAL" @LITERAL
            Rule {
                lhs: &VN_FACTOR,
                rhs: vec![&VT_LITERAL, &AC_LITERAL],
                first: vec![&VT_LITERAL],
                follow: vec![&VT_MULOP, &VT_ADDOP, &VT_AND, &VT_OR, &VT_THEN, &VT_ELSE, &VT_RPAR, &VT_COMMA, &VT_FORMAT, &VT_ENDMARK],
            },
            // 26. & <F&AC_TOR> ::= "TRUE" @TRUE
            Rule {
                lhs: &VN_FACTOR,
                rhs: vec![&VT_TRUE, &AC_TRUE],
                first: vec![&VT_TRUE],
                follow: vec![&VT_MULOP, &VT_ADDOP, &VT_AND, &VT_OR, &VT_THEN, &VT_ELSE, &VT_RPAR, &VT_COMMA, &VT_FORMAT, &VT_ENDMARK],
            },
            // 27. & <F&AC_TOR> ::= "FALSE" @FALSE
            Rule {
                lhs: &VN_FACTOR,
                rhs: vec![&VT_FALSE, &AC_FALSE],
                first: vec![&VT_FALSE],
                follow: vec![&VT_MULOP, &VT_ADDOP, &VT_AND, &VT_OR, &VT_THEN, &VT_ELSE, &VT_RPAR, &VT_COMMA, &VT_FORMAT, &VT_ENDMARK],
            },
            // 28. & <F&AC_TOR> ::= "(" <EXP> @SKIP  ")"
            Rule {
                lhs: &VN_FACTOR,
                rhs: vec![&VT_LPAR, &VN_EXP, &AC_SKIP, &VT_RPAR],
                first: vec![&VT_LPAR],
                follow: vec![&VT_MULOP, &VT_ADDOP, &VT_AND, &VT_OR, &VT_THEN, &VT_ELSE, &VT_RPAR, &VT_COMMA, &VT_FORMAT, &VT_ENDMARK],
            },
            // 29. & <F&AC_TOR> ::= "SELECT" <SELECTIONS> @SELECT
            Rule {
                lhs: &VN_FACTOR,
                rhs: vec![&VT_SELECT, &VN_SELECTIONS, &AC_SELECT],
                first: vec![&VT_SELECT],
                follow: vec![&VT_MULOP, &VT_ADDOP, &VT_AND, &VT_OR, &VT_THEN, &VT_ELSE, &VT_RPAR, &VT_COMMA, &VT_FORMAT, &VT_ENDMARK],
            },
            // 30. & <FUNCTION'> ::= "(" <LIST> @SKIP ")" @CALL
            Rule {
                lhs: &VN_FUNCTION_,
                rhs: vec![&VT_LPAR, &VN_LIST, &AC_SKIP, &VT_RPAR, &AC_CALL],
                first: vec![&VT_LPAR],
                follow: vec![&VT_MULOP, &VT_ADDOP, &VT_AND, &VT_OR, &VT_THEN, &VT_ELSE, &VT_RPAR, &VT_COMMA, &VT_FORMAT, &VT_ENDMARK],
            },
            // 31. <FUNCTION'> ::= <EXP> @PARAMETER @CALL
            Rule {
                lhs: &VN_FUNCTION_,
                rhs: vec![&VN_EXP, &AC_PARAMETER, &AC_CALL],
                first: vec![&VT_SELECT, &VT_FUNCTION, &VT_VARIABLE, &VT_INTEGER, &VT_DECIMAL, &VT_LITERAL, &VT_TRUE, &VT_FALSE, &VT_NOT, &VT_IF, ],
                follow: vec![&VT_MULOP, &VT_ADDOP, &VT_AND, &VT_OR, &VT_THEN, &VT_ELSE, &VT_RPAR, &VT_COMMA, &VT_FORMAT, &VT_ENDMARK],
            },
            // 32. <FUNCTION'> ::= @NOPARAMETER @CALL
            Rule {
                lhs: &VN_FUNCTION_,
                rhs: vec![&AC_NOPARAMETER, &AC_CALL],
                first: vec![&VT_RPAR, &VT_THEN, &VT_ELSE, &VT_OR, &VT_AND, &VT_RELOP, &VT_COMMA, &VT_ADDOP, &VT_MULOP, &VT_FORMAT, &VT_ENDMARK ],
                follow: vec![],
            },
            // 33. & <LIST> ::= <EXP> @CREATELIST <LIST'>
            Rule {
                lhs: &VN_LIST,
                rhs: vec![&VN_EXP, &AC_CREATELIST, &VN_LIST_],
                first: vec![&VT_SELECT, &VT_FUNCTION, &VT_VARIABLE, &VT_INTEGER, &VT_DECIMAL, &VT_LITERAL, &VT_TRUE, &VT_FALSE, &VT_LPAR, &VT_NOT, &VT_IF],
                follow: vec![&VT_RPAR],
            },
            // 34. & <LIST> ::= @EMPTYLIST
            Rule {
                lhs: &VN_LIST,
                rhs: vec![&AC_EMPTYLIST],
                first: vec![&VT_RPAR],
                follow: vec![&VT_RPAR],
            },
            // 35. & <LIST'> ::= "," <EXP> @INSERTLIST <LIST'>
            Rule {
                lhs: &VN_LIST_,
                rhs: vec![&VT_COMMA, &VN_EXP, &AC_INSERTLIST, &VN_LIST_],
                first: vec![&VT_COMMA],
                follow: vec![&VT_RPAR],
            },
            // 36. & <LIST'> ::= @ECHO
            Rule {
                lhs: &VN_LIST_,
                rhs: vec![&AC_ECHO],
                first: vec![&VT_RPAR],
                follow: vec![],
            },
            // 37. <SELECTIONS> ::= <SELITEM> @FIRSTITEM <SELECTIONS'>
            Rule {
                lhs: &VN_SELECTIONS,
                rhs: vec![&VN_SELITEM, &AC_FIRSTITEM, &VN_SELECTIONS_],
                first: vec![&VT_VARIABLE],
                follow: vec![&VT_MULOP, &VT_ADDOP, &VT_AND, &VT_OR, &VT_THEN, &VT_ELSE, &VT_RPAR, &VT_COMMA, &VT_FORMAT, &VT_ENDMARK],
            },
            // 38. <SELECTIONS'> ::= "," <SELITEM> @INSERTITEM <SELECTIONS'>
            Rule {
                lhs: &VN_SELECTIONS_,
                rhs: vec![&VT_COMMA, &VN_SELITEM, &AC_INSERTITEM, &VN_SELECTIONS_],
                first: vec![&VT_COMMA],
                follow: vec![&VT_MULOP, &VT_ADDOP, &VT_AND, &VT_OR, &VT_THEN, &VT_ELSE, &VT_RPAR, &VT_COMMA, &VT_FORMAT, &VT_ENDMARK],
            },
            // 39. <SELECTIONS'> ::= @ECHO
            Rule {
                lhs: &VN_SELECTIONS_,
                rhs: vec![&AC_ECHO],
                first: vec![&VT_RPAR, &VT_THEN, &VT_ELSE, &VT_OR, &VT_AND, &VT_RELOP, &VT_ADDOP, &VT_MULOP, &VT_FORMAT, &VT_ENDMARK],
                follow: vec![],
            },
            // 40. <SELITEM> ::= "VARIABLE" @VARIABLE <SELITEM'>
            Rule {
                lhs: &VN_SELITEM,
                rhs: vec![&VT_VARIABLE, &AC_VARIABLE, &VN_SELITEM_],
                first: vec![&VT_VARIABLE],
                follow: vec![&VT_MULOP, &VT_ADDOP, &VT_AND, &VT_OR, &VT_THEN, &VT_ELSE, &VT_RPAR, &VT_COMMA, &VT_FORMAT, &VT_ENDMARK],
            },
            // 41. <SELITEM'> ::= "FROM" <SELECTIONS> @SELECTITEM
            Rule {
                lhs: &VN_SELITEM_,
                rhs: vec![&VT_FROM, &VN_SELECTIONS, &AC_SELECTITEM],
                first: vec![&VT_FROM],
                follow: vec![&VT_MULOP, &VT_ADDOP, &VT_AND, &VT_OR, &VT_THEN, &VT_ELSE, &VT_RPAR, &VT_COMMA, &VT_FORMAT, &VT_ENDMARK],
            },
            // 42. <SELITEM'> ::= @ECHO
            Rule {
                lhs: &VN_SELITEM_,
                rhs: vec![&AC_ECHO],
                first: vec![&VT_RPAR, &VT_THEN, &VT_ELSE, &VT_OR, &VT_AND, &VT_RELOP, &VT_COMMA, &VT_ADDOP, &VT_MULOP, &VT_FORMAT, &VT_ENDMARK],
                follow: vec![],
            },
            // 43. <SELITEM'> ::= FORMAT <EXP> @FORMAT
            Rule {
                lhs: &VN_FMTEXP_,
                rhs: vec![&VT_FORMAT, &VN_EXP, &AC_FORMAT],
                first: vec![&VT_FORMAT],
                follow: vec![&VT_SELECT, &VT_FUNCTION, &VT_VARIABLE, &VT_INTEGER, &VT_DECIMAL, &VT_LITERAL, &VT_TRUE, &VT_FALSE, &VT_LPAR, &VT_NOT, &VT_IF],
            },
            // 44. <FMTEXP'> ::= @ECHO
            Rule {
                lhs: &VN_FMTEXP_,
                rhs: vec![&AC_ECHO],
                first: vec![&VT_SELECT, &VT_FUNCTION, &VT_VARIABLE, &VT_INTEGER, &VT_DECIMAL, &VT_LITERAL, &VT_TRUE, &VT_FALSE, &VT_LPAR, &VT_NOT, &VT_IF, &VT_ENDMARK],
                follow: vec![],
            },
       ]

    }

    fn create_m(rules: &Vec<Rule>) -> Array2D<i32> {

        let number_of_terminals= NUMBEROFTERMINALS.to_int(); // Number of terminals,
        let number_of_variables = NUMBEROFVARIABLES.to_int(); // Number of variables

        let mut m_ = Array2D::filled_with(-1, number_of_terminals, number_of_variables);

        // Fill the parsing table with the rules
        let mut number_of_rule = 0;
        for rule in rules.iter() {
            let row = rule.lhs.to_int();
            for first in rule.first.iter() {
                let col = first.to_int();
                if m_[(row, col)] >= 0 {
                    panic!("Conflict in parsing table at row {}, col {}", row, col);
                };
                m_[(row, col)] = number_of_rule; // Set the rule index
            }
            for follow in rule.follow.iter() {
                let col = follow.to_int();
                if m_[(row, col)] == -1 {
                    m_[(row, col)] = -2; // Set to -2 for follow
                }
            }
            number_of_rule += 1;
        }

        m_
    }

    pub fn create() -> Grammar {
        let rules = Grammar::create_rules();
        let m = Grammar::create_m(&rules);

        Grammar { rules, m, end_mark: &VT_ENDMARK, }
    }

    #[inline]
    pub fn get_end_mark(&self) -> &'static Tag {
        &self.end_mark
    }

    #[inline]
    pub fn get_lhs(&self, production: usize) -> &'static Tag {
        &self.rules[production].lhs
    }

    #[inline]
    pub fn get_rhs(&self, production: usize) -> &Vec<&'static Tag> {
        &self.rules[production].rhs
    }

    #[allow(dead_code)]
    pub fn get_production(&self, a: &Tag, b: &Tag) -> i32 {
        let row = a.to_int() as usize;
        let col = b.to_int() as usize;

        if row < self.m.row_len() && col < self.m.column_len() {
            return self.m[(row, col)];
        } else {
            return -1;
        }
    }
}
