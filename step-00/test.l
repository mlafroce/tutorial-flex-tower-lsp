/* 
 * Lenguage test
 */

%option noyywrap

%%
%column
[a-zA-Z]+	printf("PALABRA -> '%s' (largo %d)\n", yytext, yyleng);
[0-9]+	printf("NUMERO  -> '%s' (largo %d)\n", yytext, yyleng);
[ ]+	printf("%d ESPACIOS\n", yyleng);	
%%

int main(int argc, char** argv)
{
  yylex();
}
