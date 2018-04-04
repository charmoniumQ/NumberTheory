(module functions_sets racket/base
	(require srfi/1)
	(require memoize)
	(require "racket_specific.rkt")
	(require "lisp_utils.rkt")
	(require "k-ary.rkt")
	(require "triangle_chart.rkt")
	(require racket/file)
	(module+ test (require rackunit))

	(define (first-non-div a b)
		(find (compose not (curry k-ary-divides-pow? a b)) (rangei 0 (+ 1 b) 2)))

	(define (first-div a b)
		(find (curry k-ary-divides-pow? a b) (rangei 1 (+ 1 b) 2)))

	(define/memo (k-characteristic a b)
		(if (infinitary-divides-pow? a b) (first-div a b) (first-non-div a b)))


	;; (module+ main
		;; (define sq-size 16)
		;; (define text-size 8)
		;; (define coloration 0.5)
		;; (save-grid k-characteristic (lambda (val n level) #t) coloration 10 sq-size text-size "output/small1_triangle.png")
		;; (save-grid k-characteristic (lambda (val n level) #t) coloration 30 sq-size text-size "output/small2_triangle.png"))

	(module+ main
		(define max-level 250)
		(define sq-size 4)
		(define text-size 0)
		(define coloration 0.5)
		;; (save-grid k-characteristic (lambda (val n level) (odd? val) ) coloration max-level sq-size text-size "output/triangle_odd.png")
		;; (save-grid k-characteristic (lambda (val n level) (even? val)) coloration max-level sq-size text-size "output/triangle_even.png")
		(save-grid k-characteristic (lambda (val n level) #t         ) coloration max-level sq-size text-size "output/triangle.png")
		(display-to-file (as-csv
			(map
				(lambda (b)
					(display (format "level ~a~n" b))
					(map
						(lambda (a) (number->string (k-characteristic a b)))
						(rangei 0 b)))
				(rangei 0 max-level))) "output/triangle.csv"))

	(provide k-characteristic))
