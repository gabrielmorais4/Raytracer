##
## EPITECH PROJECT, 2021
## makefile
## File description:
## makefile
##

NAME = raytracer

SEVER_SRC = server/*.c

all:
	cargo build

run:
	cargo run

clean:
	rm -f *~ | rm -f *.o

fclean: clean
	rm -f $(NAME)

re:	fclean all
