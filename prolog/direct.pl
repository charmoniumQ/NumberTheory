
% get divisors of N, O(N)
divisors(N, Ds) :-
    N > 0,
    divisors(N, 1, Ds).
divisors(N, D, [N]) :-
    D >= N.
divisors(N, D, [D|Ds]) :-
    D < N,
    0 is mod(N, D),
    D1 is D+1,
    divisors(N, D1, Ds).
divisors(N, D, Ds) :-
    D < N,
    \+ (0 is mod(N, D)),
    D1 is D+1,
    divisors(N, D1, Ds).

% Base case of 'derivative'
deriv(A, 0, N, Rs) :-
    call(A, N, Rs).

% recursive case of kth 'derivative' of A at N
deriv(A, K, N, Rs) :-
    K > 0,
    K1 is K-1,
    divisors(N, Ds),
    include(deriv_member(deriv(A,K1),N), Ds, Rs).

% helper function for 'derivative'
deriv_member(F,N,D) :-
    call(F,D,As),
    ND is N/D,
    call(F, ND, Bs),
    intersection(As,Bs,[1]).

% infinitary division, closed form
theta(1, [1]).
theta(N, Rs) :-
    divisors(N, Ds),
    include(theta_member(N), Ds, Rs).

% infinitary division member predicate, coinductive case 1
%
% This case would infinitely recurse because the second term in the intersection
% is theta(N), but the first term is theta(1), which makes the intersection {1}
% regardless of theta(N).
theta_member(_,1).

% infinitary division member predicate, coinductive case 2.
%
% This would normally infinitely recurse, becasue it would ask for theta(N),
% but theta(N) needs theta_member(N,N). However, the second term in the
% intersect is theta(1), so the intersect will be {1} regardless of theta(N).
theta_member(N,N).

theta_member(N, D) :-
    theta(D, Ds),
    ND is N/D,
    theta(ND, NDs),
    intersection(Ds, NDs, [1]).
