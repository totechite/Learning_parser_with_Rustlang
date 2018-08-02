# Learning_parser_with_Rustlang
Learning "Language Implementation Patterns" published by O'Reilly in practice. https://www.oreilly.co.jp/books/9784873115320/

## Sample
This program actions lexing the code that had very easy sintax and structed a few elements like include only SquereBracket, Camma and any string.   

Here is a result when executing example code.   

```
//sample.md:
[a,b,C,[abc,[a,bc],ab]]

 |
 V

Token { char_type: L_BRACKET, text: "[" }
Token { char_type: NAME, text: "a" }
Token { char_type: COMMA, text: "," }
Token { char_type: NAME, text: "b" }
Token { char_type: COMMA, text: "," }
Token { char_type: NAME, text: "C" }
Token { char_type: COMMA, text: "," }
Token { char_type: L_BRACKET, text: "[" }
Token { char_type: NAME, text: "abc" }
Token { char_type: COMMA, text: "," }
Token { char_type: L_BRACKET, text: "[" }
Token { char_type: NAME, text: "a" }
Token { char_type: COMMA, text: "," }
Token { char_type: NAME, text: "bc" }
Token { char_type: R_BRACKET, text: "]" }
Token { char_type: COMMA, text: "," }
Token { char_type: NAME, text: "ab" }
Token { char_type: R_BRACKET, text: "]" }
Token { char_type: R_BRACKET, text: "]" }
Token { char_type: EOF, text: "<EOF>" }
```

## To Do
- Implement Parser