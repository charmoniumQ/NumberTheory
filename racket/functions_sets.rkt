(module functions_sets racket/base
	(require srfi/1)
	(require "lisp_utils.rkt")
	(require "racket_specific.rkt")
	(module+ test (require rackunit))

	; https://en.wikipedia.org/wiki/Iverson_bracket
	; #t -> 1 and #f -> 0
	(define (iverson s) (if s 1 0))

	(define (indicator set)
		(lambda (x) (iverson (member x set))))
	(module+ test (check-equal? 1 ((indicator '(1 2)) 2)))
	(module+ test (check-equal? 0 ((indicator '(1 2)) 3)))

	(define (const x0) (lambda (x) x0))
	(module+ test (check-equal? 1 ((const 1) 10)))

	(define (identity x) x)
	(module+ test (check-equal? 10 (identity 10)))

	(define (sum lst) (foldl + 0 lst))
	(module+ test (check-equal? 10 (sum (rangei 0 4))))

	; The following function tests to see if two functions are equal for the
	; first 1000 natural numbers
	(define (fun= f g [domain (rangei 1 1000)])
		(list= =
			(map f domain)
			(map g domain)))
	(module+ test (check-true
		(fun= (indicator '(3)) (lambda (x) (iverson (= x 3))))))

	(define intersection (curry lset-intersection equal?))
	(module+ test (check-equal?
		'(1 3)
		(intersection (rangei 1 6) '(0 1 3))))

	(define seteq (curry lset= =))
	(define subseteq (curry lset<= =))
	(define subset
		(lambda args
			(and
				(apply subseteq args)
				(not (apply seteq args)))))
	(define superseteq (rev-args subseteq))
	(define superset (rev-args subset))
	(module+ test (check-true (superseteq '(1 2 3) '(1 2))))
	(module+ test (check-true (superseteq '(1 2 3) '(1 2 3))))
	(module+ test (check-false (superseteq '(1 2 3) '(1 2 3 4))))
	(module+ test (check-true (superset '(1 2 3) '(1 2))))
	(module+ test (check-false (superset '(1 2 3) '(1 2 3))))

	; efficiently compute the intersection of two already-sorted sets
	(define (sorted-set-intersection seta setb)
		(define (loop seta setb acc)
			(if (or (null? seta) (null? setb))
				acc
				(if (< (car seta) (car setb))
					(loop (cdr seta) setb acc)
					(if (> (car seta) (car setb))
						(loop seta (cdr setb) acc)
						(loop (cdr seta) (cdr setb)
								(append acc (list (car seta))))))))
		(loop seta setb '()))
	(module+ test (check-equal?
		(sorted-set-intersection '(1 2 3) '(1 3 4))
		'(1 3)))

	; efficiently compute the intersection of two already-sorted sets
	(define (sorted-set-intersection-max seta setb)
		(define (loop seta setb)
			(if (or (null? seta) (null? setb))
				0
				(if (< (car seta) (car setb))
					(loop seta (cdr setb))
					(if (> (car seta) (car setb))
						(loop (cdr seta) setb)
						(car seta)))))
		(loop (reverse seta) (reverse setb)))
	(module+ test (check-equal?
		3
		(sorted-set-intersection-max '(1 2 3) '(1 3 4))))

	(provide iverson indicator fun= identity const sum
			 intersection seteq subseteq subset superseteq superset sorted-set-intersection sorted-set-intersection-max))
