/* 
 * Lenguage test
 */

%option noyywrap

%{
#include "barba-lang.h"
%}

%%
[a-zA-Z]+ lexer_push_token(rust_parser, PALABRA, yytext, yylineno, yycolumn - yyleng);
[0-9]+  lexer_push_token(rust_parser, NUMERO, yytext, yylineno, yycolumn - yyleng);
[ ]+  {}
\n  yycolumn=0; ++yylineno;
%%

void parse(void* rust_parser, int fd)
{
  yycolumn=0;
  yylineno=1;
  if (fd)
  {
    yyin = fdopen(fd, "r");
  }
  yylex(rust_parser);
}
