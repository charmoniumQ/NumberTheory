(module lisp_utils racket/base
	(require srfi/1)
	(require "racket_specific.rkt")
	(module+ test (require rackunit))

	(define (join list1 elem)
		(if (null? list1)
			list1
			(if (null? (cdr list1))
				list1
				(append (list (car list1) elem) (join (cdr list1) elem)))))
	(module+ test
		(check-equal? (join '(1 2 3 4) 10) '(1 10 2 10 3 10 4)))
	(module+ test
		(check-equal? (join '(1) 10) '(1)))
	(module+ test
		(check-equal? (join '() 10) '()))

	; Home on the range
	(define (range start stop . step)
		(if (null? step)
			(range start stop 1)
			(iota (- stop start) start (car step))))

	; i for inclusive
	(define (rangei start stop . step)
		(if (null? step)
			(rangei start stop 1)
			(iota (+ 1 (- stop start)) start (car step))))
	(module+ test
		(check-equal? '(1 2 3 4) (rangei 1 4)))

	(define (rev-args f)
		(lambda args (apply f (reverse args))))
	(module+ test
		(check-eqv? 1 ((rev-args -) 2 3)))

	; '(a b c) -> '((0 a) (1 b) (2 c))
	(define (enumerate lst) (zip (range 0 (length lst)) lst))
	(module+ test (check-equal?
		'((0 4) (1 7) (2 11))
		(enumerate '(4 7 11))))

	; Inverse of enumerate, given size
	(define (unumerate lst n)
		(map (lambda (i)
			(let ([it (assq i lst)])
				(if it
					(second it)
					0)))
		(range 0 n)))
	(module+ test (check-equal?
		'(4 7 3 0 3 0)
		(unumerate '((1 7) (4 3) (0 4) (2 3)) 6)))

	; Inverse of enumerate without size
	(define (unumerate* lst)
		(if (null? lst)
			'()
			(unumerate lst (+ 1 (apply max (map first lst))))))
	(module+ test (check-equal?
		'(4 7 3 0 3)
		(unumerate* '((1 7) (4 3) (0 4) (2 3)))))

	;; (define (filter-map f lst)
	;; 	(let
	;; 		([out (filter (compose not false?) (map f lst))])
	;; 		(if (null? out) #f out)))
	;; (module+ test (check-equal?
	;; 	(filter-map (lambda (x) (if (even? x) x #f)) (rangei 0 10))
	;; 	'(0 2 4 6 8 10)))


	(define (nested-map f lst n)
		(if (= n 1)
			(map f lst)
			(map (lambda (row) (nested-map f row (- n 1))) lst)))
	(module+ test (check-equal?
			(nested-map (lambda (x) (* 2 x)) '((1 2) (3 4) (5 6)) 2)
			'((2 4) (6 8) (10 12))))

	(define (as-csv lst)
		(string-append (string-join (map (lambda (row) (string-join row ",")) lst) "\n") "\n"))
	(module+ test (check-equal?
		(as-csv '(("a" "b") ("c" "d")))
		"a,b\nc,d\n"))

	(define (from-csv str)
		(map
			(lambda (row) (string-split row ","))
			(string-split str "\n")))
	(module+ test (check-equal?
		'(("a" "b") ("c" "d"))
		(from-csv "a,b\nc,d\n")))

	(define (weird-mod n m)
		(if (= m 0)
			1
			(let ([r (remainder n m)])
				(if (= r 0) m r))))

	; Apply f to x n times
	(define (repeatedly f x n)
		(if (= n 0)
			x
			(repeatedly f (f x) (- n 1))))
	(module+ test (check-equal?
		(+ 1 (* 3 10))
		(repeatedly (lambda (n) (+ n 3)) 1 10)))

	(provide join range rangei rev-args enumerate unumerate unumerate* nested-map weird-mod as-csv from-csv repeatedly))
