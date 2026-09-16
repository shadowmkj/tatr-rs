# Parsing Algorithm

- STATUS: OPEN
- PRIORITY: 100
- TAGS: parser,grammar

Implement recursive descent parser for this grammar
```
(* Entry rule *)
Query        ::= OrExpr

(* Precedence levels: OR (lowest) -> AND -> NOT (highest) *)
OrExpr       ::= AndExpr ( "or" AndExpr )*
AndExpr      ::= NotExpr ( "and" NotExpr )*
NotExpr      ::= "not" NotExpr
               | Tag

(* Lexical rules *)
Tag          ::= "." IDENTIFIER
IDENTIFIER   ::= [a-zA-Z_] [a-zA-Z0-9_-]*
```


