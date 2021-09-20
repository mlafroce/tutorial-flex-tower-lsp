#define YY_DECL char *yylex(void* rust_parser)
#define YY_USER_ACTION yycolumn += yyleng;
size_t yycolumn = 0;

enum TokenType { PALABRA, NUMERO };

void lexer_push_token(void* rust_parser, enum TokenType token_type, char* label, size_t yylineno, size_t yycolumn);
