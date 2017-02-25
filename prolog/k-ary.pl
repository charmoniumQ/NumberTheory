%% :- use_module(library(tabling)).
%% :- table divisors/3

% TODO: place cuts

divides(A, C, 0) :- !, A =< C.
divides(A, C, K) :-
	K \= 0, % Is this necesssary if the k = 0 rule is before?
	!,
	B is C - A,
	Km1 is K - 1,
	coprime(A, B, Km1).

coprime(A, B, K) :- gcd(A, B, K, 0).

gcd(A, B, K, X) :-
	divisors(A, K, AFactors),
	divisors(B, K, BFactors),
	intersection(AFactors, BFactors, CommonDivisors),
	last(CommonDivisors, X).

divisors(A, K, Xs) :- setof(X, (between(0, A, X), divides(X, A, K)), Xs).

intersection(_, [], []) :- !.
intersection([], _, []) :- !.
intersection([Head|TailA], [Head|TailB], [Head|TailInt]) :- !, intersection(TailA, TailB, TailInt).
intersection([HeadA|TailA], [HeadB|TailB], X) :-
	HeadA > HeadB,
	!,
	intersection([HeadA|TailA], TailB, X).
intersection([HeadA|TailA], [HeadB|TailB], X) :-
	HeadA < HeadB,
	!,
	intersection(TailA, [HeadB|TailB], X).

last([_|Tail], X) :- last(Tail, X).
last([Elem], Elem).
