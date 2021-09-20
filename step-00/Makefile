LEX=lex

all: lexer

lex.yy.c: test.l
	$(LEX) $?

lexer: lex.yy.c
	$(CC) $? -lfl -o $@

clean:
	$(RM) lex.yy.c lexer

.PHONY: all clean
