##
## EPITECH PROJECT, 2021
## makefile
## File description:
## makefile
##

NAME = raytracer

all:
	cargo build
	mv ./target/debug/$(NAME) .

run:
	cargo run

clean:
	rm -f *~ | rm -f *.o

fclean: clean
	rm -f $(NAME)

raytracer:
	cargo build
	mv ./target/debug/$(NAME) .

re:	fclean all
