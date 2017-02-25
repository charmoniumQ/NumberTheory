(module derivative racket/base

	; Definition of divisor set of a prime power under special division
	(define (s-divisors-pow s-divides-pow? a)
		(filter
			(curryr s-divides-pow? a)
			(rangei 0 a)))

	; Definition of GCD of two prime powers under special division
	(define (s-gcd-pow s-divides-pow? a b)
		(sorted-set-intersection-max
			(s-divisors-pow a k)
			(s-divisors-pow b k)))

	; Definition of coprime of two prime powers under special division
	(define (s-coprime-pow? s-divides-pow? a b)
		(= 0 (s-gcd-pow s-divides-pow? a b)))

	; Definition of derivative of a division
	(define (d s-divides-pow?)
		(lambda (a b) (and (<= a b) (s-coprime-pow? a (- b a)))))

	(define (divides-pow? a b) (<= a b))

	(provides d divides-pow? s-divisors-pow s-gcd-pow))
