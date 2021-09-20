/* 
 * Lenguage test
 */

%option noyywrap

%%
[a-zA-Z]+	printf("PALABRA -> '%s' (largo %d)\n", yytext, yyleng);
[0-9]+	printf("NUMERO  -> '%s' (largo %d)\n", yytext, yyleng);
[ ]+	printf("%d ESPACIOS\n", yyleng);	
%%

void parse()
{
  yylex();
}
