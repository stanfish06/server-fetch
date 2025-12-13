all: libbindings.a

bindings.o: bindings.c
	gcc -c -o bindings.o bindings.c

libbindings.a: bindings.o
	ar rcs libbindings.a bindings.o	

clean:
	rm *.o *.a
